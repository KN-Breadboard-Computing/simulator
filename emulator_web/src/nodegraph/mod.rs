use egui::Pos2;
use emulator_core::graph::Graph;
use slotmap::SecondaryMap;

use self::{
    cables::{CableId, CablesGraph},
    components::Component,
    components::ComponentId,
    components::LogicalComponent,
};

pub mod cables;
pub mod components;
pub mod drawables;
pub mod transform;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct NodeGraph {
    pub components: SecondaryMap<ComponentId, Component>,
    pub logical_graph: Graph,
    pub cables: CablesGraph,
}

impl NodeGraph {
    pub fn new() -> Self {
        Self {
            components: SecondaryMap::new(),
            cables: CablesGraph::new(),
            logical_graph: Graph::new(),
        }
    }

    pub fn add_component(&mut self, logic_comp: LogicalComponent, pos: Pos2) -> ComponentId {
        let comp = Component::from_logical_component(&logic_comp, pos);
        let id = self.logical_graph.add_comp(logic_comp).into();
        self.components.insert(id, comp);
        id
    }

    pub fn connect_cable(&mut self, cable_id: CableId) {
        let &first = self.cables.map[cable_id].points.first().unwrap();

        let first_component = self
            .components
            .iter()
            .find(|(_, comp)| comp.contains(first))
            .map(|(id,_)| id);

        let Some(first_component) = first_component else {return;};

        let &last = self.cables.map[cable_id].points.last().unwrap();

        let last_component = self
            .components
            .iter()
            .find(|(_, comp)| comp.contains(last))
            .map(|(id,_)| id);

        let Some(last_component) = last_component else {return;};

        self.logical_graph.add_conn(first_component, 0, last_component, 0);
    }
}
