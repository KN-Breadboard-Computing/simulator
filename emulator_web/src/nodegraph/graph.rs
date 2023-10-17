use std::{array::from_fn, collections::HashSet};

use egui::{Pos2, Vec2};
use emulator_core::{
    components::{simple::Fork, ComponentBehaviour},
    graph::{id::ComponentId, Graph},
};
use log::warn;
use slotmap::{SecondaryMap, SlotMap};

use crate::{
    components::{
        add_data::get_size,
        registry::{ComponentRegistry, ComponentRid},
    },
    util::{IRect, IVec2},
};

use super::{
    cables::{Cable, CableEnd, CableId, CableIntersection},
    components::{ComponentIntersection, ComponentNode},
    transform::NodeGraphBounds,
};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct NodeGraph {
    pub graph: Graph,
    pub cables: SlotMap<CableId, Cable>,
    pub components: SecondaryMap<ComponentId, ComponentNode>,
    pub bounds: NodeGraphBounds,
}

impl NodeGraph {
    pub fn new() -> Self {
        Self {
            graph: Graph::new(),
            cables: SlotMap::with_key(),
            components: SecondaryMap::new(),
            bounds: NodeGraphBounds::default(),
        }
    }

    pub fn cable(&self, id: CableId) -> &Cable {
        &self.cables[id]
    }

    pub fn cable_mut(&mut self, id: CableId) -> &mut Cable {
        &mut self.cables[id]
    }

    pub fn comp(&self, id: ComponentId) -> &ComponentNode {
        &self.components[id]
    }

    pub fn comp_mut(&mut self, id: ComponentId) -> &mut ComponentNode {
        &mut self.components[id]
    }

    pub fn new_cable(&mut self, start: IVec2) -> CableId {
        self.cables.insert(Cable::starting_in(start))
    }

    pub fn new_component(
        &mut self,
        pos: IVec2,
        entry_rid: ComponentRid,
        registry: &ComponentRegistry,
    ) -> ComponentId {
        let component = (registry
            .entry(entry_rid)
            .expect("ComponentRid not found")
            .create)();
        let size = get_size(&component);
        let node = ComponentNode::new(IRect::new(pos - size / 2, size), entry_rid)
            .with_default_slots(component.input_size(), component.output_size());

        let id = self.graph.add_comp(component).into();
        self.components.insert(id, node);

        id
    }
}

impl NodeGraph {
    pub fn cables_intersecting(
        &self,
        pos: Pos2,
        epsilon: f32,
    ) -> impl Iterator<Item = (CableId, CableIntersection)> + '_ {
        self.cables
            .iter()
            .filter_map(move |(id, cable)| cable.intersect_point(pos, epsilon).map(|int| (id, int)))
    }

    pub fn cables_intersecting_exact(
        &self,
        pos: IVec2,
    ) -> impl Iterator<Item = (CableId, CableIntersection)> + '_ {
        self.cables
            .iter()
            .filter_map(move |(id, cable)| cable.intersect_point_exact(pos).map(|int| (id, int)))
    }

    pub fn components_intersecting(
        &self,
        pos: Pos2,
        epsilon: f32,
    ) -> impl Iterator<Item = (ComponentId, ComponentIntersection)> + '_ {
        self.components
            .iter()
            .filter_map(move |(id, comp)| comp.intersection_test(pos, epsilon).map(|int| (id, int)))
    }

    pub fn components_intersecting_exact(
        &self,
        pos: IVec2,
    ) -> impl Iterator<Item = (ComponentId, ComponentIntersection)> + '_ {
        self.components
            .iter()
            .filter_map(move |(id, comp)| comp.intersection_test_exact(pos).map(|int| (id, int)))
    }
}

impl NodeGraph {
    pub fn remove_cable(&mut self, id: CableId) -> Option<Cable> {
        if self.cables.contains_key(id) {
            self.clean_neighbours(id);
            let cable = self.cables.remove(id).unwrap();
            Some(cable)
        } else {
            None
        }
    }

    pub fn detach_cable_conn(&mut self, id: CableId) {
        for end in [CableEnd::First, CableEnd::Last] {
            if let Some(comp_id) = self.cable_mut(id).conn_comp_at_end_mut(end).take() {
                for (i, cable_conn_id) in
                    &mut self.components[comp_id].input_cables.iter_mut().enumerate()
                {
                    if cable_conn_id == &Some(id) {
                        self.graph.remove_conn_to(comp_id, i);
                        *cable_conn_id = None;
                    }
                }

                for (o, cable_conn_id) in &mut self.components[comp_id]
                    .output_cables
                    .iter_mut()
                    .enumerate()
                {
                    if cable_conn_id == &Some(id) {
                        self.graph.remove_conn_from(comp_id, o);
                        *cable_conn_id = None;
                    }
                }
            }
        }
    }

    pub fn find_and_attach_cable_conn(
        &mut self,
        id: CableId,
    ) -> [Option<(ComponentId, ComponentIntersection)>; 2] {
        let mut ints = from_fn(|_| None);
        for end in [CableEnd::First, CableEnd::Last] {
            if self.cable(id).neighbour_at_end(end).is_empty() {
                let &end_pos = self.cable(id).point_at_end(end);
                let maybe_int = self.components_intersecting_exact(end_pos).next();
                if let Some((comp_id, slot)) = maybe_int {
                    match slot {
                        ComponentIntersection::InputSlot(i) => {
                            self.comp_mut(comp_id).input_cables[i] = Some(id);
                        }
                        ComponentIntersection::OutputSlot(o) => {
                            self.comp_mut(comp_id).output_cables[o] = Some(id);
                        }
                        ComponentIntersection::Inside => {
                            continue;
                        }
                    }
                    *self.cable_mut(id).conn_comp_at_end_mut(end) = Some(comp_id);
                    ints[end as usize] = Some((comp_id, slot));
                }
            }
        }
        ints
    }

    pub fn remove_cable_and_fix(&mut self, id: CableId) {
        self.detach_cable_conn(id);
        let removed = self.remove_cable(id).unwrap();
        for &n_id in removed.neighbours() {
            self.fix_after_moving_cable(n_id);
        }
    }

    pub fn fix_after_moving_cable(&mut self, id: CableId) {
        self.repair_cable_neighbours(id);
        let group = self.travel_cable_group(id);
        let mut group_inputs = Vec::new();
        let mut group_outputs = Vec::new();
        for &cable_id in &group {
            self.detach_cable_conn(cable_id);
        }
        for &cable_id in &group {
            let found = self.find_and_attach_cable_conn(cable_id);
            for (comp_id, int) in found.into_iter().flatten() {
                match int {
                    ComponentIntersection::InputSlot(i) => {
                        group_outputs.push((comp_id, i));
                    }
                    ComponentIntersection::OutputSlot(o) => {
                        group_inputs.push((comp_id, o));
                    }
                    ComponentIntersection::Inside => {}
                }
            }
        }

        if !group_inputs.is_empty() && !group_outputs.is_empty() {
            let fork = Fork::new(group_inputs.len() as u8, group_outputs.len() as u8);
            let fork_id = self.graph.add_comp(fork);
            for (i, (c_id, o)) in group_inputs.into_iter().enumerate() {
                self.graph.add_conn(c_id, o, fork_id, i);
            }
            for (o, (c_id, i)) in group_outputs.into_iter().enumerate() {
                self.graph.add_conn(fork_id, o, c_id, i);
            }
            for &cable_id in &group {
                self.cable_mut(cable_id).fork = Some(fork_id);
            }
        }
    }

    pub fn clean_neighbours(&mut self, id: CableId) -> Vec<CableId> {
        let neighbours = self.cable(id).neighbours[0]
            .iter()
            .copied()
            .chain(self.cable(id).neighbours[1].iter().copied())
            .collect();

        for &n_id in &neighbours {
            self.cable_mut(n_id).neighbours[0].remove(&id);
            self.cable_mut(n_id).neighbours[1].remove(&id);
        }

        neighbours
    }

    ///TODO
    ///Crash, kiedy jest pętla i spróbujemy ją przesunąć
    pub fn repair_cable_neighbours(&mut self, id: CableId) {
        if self.cable(id).not_a_line() {
            return;
        }

        self.clean_neighbours(id);

        for end in [CableEnd::First, CableEnd::Last] {
            let &pos = self.cable(id).point_at_end(end);

            let new_affected = self
                .cables_intersecting_exact(pos)
                .filter(|&(i, _)| i != id)
                .collect::<Vec<_>>();

            let mut new_neighbours = HashSet::new();

            for (n_id, res) in new_affected {
                let p = match res {
                    CableIntersection::OnSegment(seg) => {
                        self.cable_mut(n_id).points.insert(seg + 1, pos);
                        seg + 1
                    }
                    CableIntersection::OnPoint(i) => i,
                };
                if p != 0 && p != self.cable(n_id).points.len() - 1 {
                    let [new_first, new_second] = self.split(n_id, p);
                    self.cables[new_first].neighbours[1].insert(id);
                    self.cables[new_second].neighbours[0].insert(id);
                    new_neighbours.extend([new_first, new_second]);
                } else {
                    if p == 0 {
                        self.cables[n_id].neighbours[0].insert(id);
                    } else {
                        self.cables[n_id].neighbours[1].insert(id);
                    }
                    new_neighbours.insert(n_id);
                }
            }

            *self.cables[id].neighbour_at_end_mut(end) = new_neighbours;
        }
    }

    pub fn split(&mut self, id: CableId, point_id: usize) -> [CableId; 2] {
        let Cable {
            mut points,
            neighbours: [start_neighbours, end_neighbours],
            ..
        } = self.remove_cable(id).unwrap();

        points.insert(point_id, points[point_id]);

        let (first_points, second_points) = points.split_at(point_id + 1);

        let first_cable = Cable {
            points: first_points.to_vec(),
            ..Default::default()
        };
        let second_cable = Cable {
            points: second_points.to_vec(),
            ..Default::default()
        };

        let first_id = self.cables.insert(first_cable);
        let second_id = self.cables.insert(second_cable);

        self.cables[first_id].neighbours[1].insert(second_id);
        self.cables[second_id].neighbours[0].insert(first_id);

        for &n_id in &start_neighbours {
            for n_end in [CableEnd::First, CableEnd::Last] {
                if let Some(f_end) =
                    self.cables[n_id].end_connects_to(n_end, &self.cables[first_id])
                {
                    self.cables[n_id].neighbours[n_end as usize].insert(first_id);
                    self.cables[first_id].neighbours[f_end as usize].insert(n_id);
                }
            }
        }

        for &n_id in &end_neighbours {
            for n_end in [CableEnd::First, CableEnd::Last] {
                if let Some(s_end) =
                    self.cables[n_id].end_connects_to(n_end, &self.cables[second_id])
                {
                    self.cables[n_id].neighbours[n_end as usize].insert(second_id);
                    self.cables[second_id].neighbours[s_end as usize].insert(n_id);
                }
            }
        }

        [first_id, second_id]
    }

    ///TODO
    pub fn merge(&mut self, id_a: CableId, id_b: CableId) {}

    pub fn move_cable_point(&mut self, id: CableId, point_id: usize, new_pos: IVec2) -> usize {
        self.cables[id].move_point_aligned(point_id, new_pos)
    }

    pub fn travel_cable_group(&self, id: CableId) -> HashSet<CableId> {
        let mut stack = Vec::new();
        let mut set = HashSet::new();
        stack.push(id);
        set.insert(id);

        while let Some(id) = stack.pop() {
            for &n_id in self.cable(id).neighbours() {
                if !set.contains(&n_id) {
                    stack.push(n_id);
                    set.insert(n_id);
                }
            }
        }

        set
    }
}

impl NodeGraph {
    pub fn remove_comp(&mut self, comp_id: ComponentId) {
        for output_cable in &mut self.components[comp_id].output_cables {
            if let Some(cable_id) = output_cable.take() {
                for end in [CableEnd::First, CableEnd::Last] {
                    if &Some(comp_id) == self.cables[cable_id].conn_comp_at_end(end) {
                        *self.cables[cable_id].conn_comp_at_end_mut(end) = None
                    }
                }
            }
        }
        for input_cable in &mut self.components[comp_id].input_cables {
            if let Some(cable_id) = input_cable.take() {
                for end in [CableEnd::First, CableEnd::Last] {
                    if &Some(comp_id) == self.cables[cable_id].conn_comp_at_end(end) {
                        *self.cables[cable_id].conn_comp_at_end_mut(end) = None
                    }
                }
            }
        }
        self.graph.remove_comp(comp_id);
        self.components.remove(comp_id);
    }
}

impl Default for NodeGraph {
    fn default() -> Self {
        Self::new()
    }
}
