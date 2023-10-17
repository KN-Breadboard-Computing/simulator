use std::collections::HashSet;

use egui::{Color32, Pos2, Vec2};
use emulator_core::{graph::id::{TypedId, ComponentId}, components::simple::Fork};
use slotmap::new_key_type;

use crate::util::{ivec2, IRect, IVec2};

new_key_type! {pub struct CableId;}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Cable {
    pub points: Vec<IVec2>,
    pub neighbours: [HashSet<CableId>; 2],
    pub fork: Option<TypedId<Fork>>,
    pub conn_comp: [Option<ComponentId>; 2],
    pub color: Color32,
    pub highlight_level: u8,
}

#[derive(Debug, Clone, Copy)]
pub enum CableEnd {
    First = 0,
    Last = 1,
}

impl CableEnd {
    pub fn opposite(&self) -> Self {
        match self {
            CableEnd::First => CableEnd::Last,
            CableEnd::Last => CableEnd::First,
        }
    }
}

impl Cable {
    pub const DEFAULT_COLOR: Color32 = Color32::WHITE;
    pub const ACTIVATED_COLOR: Color32 = Color32::LIGHT_GREEN;

    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            neighbours: [HashSet::new(), HashSet::new()],
            fork: None,
            conn_comp: [None; 2],
            color: Self::DEFAULT_COLOR,
            highlight_level: 0,
        }
    }

    pub fn starting_in(point: IVec2) -> Self {
        let mut new = Self::new();
        new.points = vec![point, point];
        new
    }

    pub fn not_a_line(&self) -> bool {
        self.points.len() < 2
    }

    pub fn point_id_at_end(&self, end: CableEnd) -> usize {
        match end {
            CableEnd::First => 0,
            CableEnd::Last => self.points.len() - 1,
        }
    }

    pub fn point_at_end(&self, end: CableEnd) -> &IVec2 {
        match end {
            CableEnd::First => self.points.first().unwrap(),
            CableEnd::Last => self.points.last().unwrap(),
        }
    }
    pub fn point_at_end_mut(&mut self, end: CableEnd) -> &mut IVec2 {
        match end {
            CableEnd::First => self.points.first_mut().unwrap(),
            CableEnd::Last => self.points.last_mut().unwrap(),
        }
    }

    pub fn neighbour_at_end(&self, end: CableEnd) -> &HashSet<CableId> {
        &self.neighbours[end as usize]
    }

    pub fn neighbour_at_end_mut(&mut self, end: CableEnd) -> &mut HashSet<CableId> {
        &mut self.neighbours[end as usize]
    }

    pub fn conn_comp_at_end(&self, end: CableEnd) -> &Option<ComponentId> {
        &self.conn_comp[end as usize]
    }

    pub fn conn_comp_at_end_mut(&mut self, end: CableEnd) -> &mut Option<ComponentId> {
        &mut self.conn_comp[end as usize]
    }

    pub fn end_connects_to(&self, end: CableEnd, to: &Self) -> Option<CableEnd> {
        let point = self.point_at_end(end);
        [CableEnd::First, CableEnd::Last]
            .into_iter()
            .find(|&other_end| to.point_at_end(other_end) == point)
    }

    pub fn neighbours(&self) -> impl Iterator<Item = &CableId> + '_ {
        self.neighbours[0].iter().chain(self.neighbours[1].iter())
    }

    pub fn clean_flat_points(&mut self) {
        let mut i = 1;
        while i <= self.points.len().saturating_sub(2) {
            let [a,b,c] = self.points[i-1..=i+1] else { unreachable!() };
            if (a.x == b.x && b.x == c.x) || (a.y == b.y && b.y == c.y) {
                self.points.remove(i);
            } else {
                i += 1;
            }
        }
    }

    pub fn reverse(&mut self) {
        self.points.reverse();
        self.neighbours.reverse();
    }
}

impl Default for Cable {
    fn default() -> Self {
        Self::new()
    }
}

impl Cable {
    pub fn move_point_aligned(&mut self, id: usize, new_pos: IVec2) -> usize {
        let mut id = id;

        self.points[id] = new_pos;

        for neigh_offset in [-1, 1] {
            let neigh_id = id as i32 + neigh_offset;
            if neigh_id < 0 || neigh_id >= self.points.len() as i32 {
                continue;
            }
            let mut neigh_id = neigh_id as usize;

            if neigh_id == 0 {
                self.points.insert(1, self.points[0]);
                neigh_id += 1;
                id += 1;
            }

            if neigh_id == self.points.len() - 1 {
                self.points
                    .insert(self.points.len() - 2, self.points[self.points.len() - 1]);
            }

            let neigh_neigh_id = (neigh_id as i32 + neigh_offset) as usize;

            let point = self.points[id];
            let neigh = self.points[neigh_id];
            let neigh_neigh = self.points[neigh_neigh_id];

            if point != neigh {
                if neigh == neigh_neigh {
                    if (point.x - neigh.x).abs() > (point.y - neigh.y).abs() {
                        self.points[neigh_id].x = point.x;
                    } else {
                        self.points[neigh_id].y = point.y;
                    }
                } else if neigh.x == neigh_neigh.x {
                    self.points[neigh_id].y = point.y;
                } else {
                    self.points[neigh_id].x = point.x;
                }
            }
        }

        id
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CableIntersection {
    OnSegment(usize),
    OnPoint(usize),
}

impl Cable {
    pub fn intersect_point(&self, pos: Pos2, epsilon: f32) -> Option<CableIntersection> {
        if self.not_a_line() {
            return None;
        }

        for (id, point) in self.points.iter().enumerate() {
            if point.equals_with_rounding(pos, epsilon) {
                return Some(CableIntersection::OnPoint(id));
            }
        }

        for (id, window) in self.points.windows(2).enumerate() {
            let &[start, end] = window else { unreachable!() };

            let min = ivec2(start.x.min(end.x), start.y.min(end.y));
            let max = ivec2(start.x.max(end.x), start.y.max(end.y));

            let rect = IRect::new(min, max - min);

            if rect.contains_with_rounding(pos, epsilon) {
                return Some(CableIntersection::OnSegment(id));
            }
        }

        None
    }

    pub fn intersect_point_exact(&self, pos: IVec2) -> Option<CableIntersection> {
        if self.not_a_line() {
            return None;
        }

        for (id, point) in self.points.iter().enumerate() {
            if point == &pos {
                return Some(CableIntersection::OnPoint(id));
            }
        }

        for (id, window) in self.points.windows(2).enumerate() {
            let &[start, end] = window else { unreachable!() };

            let min = ivec2(start.x.min(end.x), start.y.min(end.y));
            let max = ivec2(start.x.max(end.x), start.y.max(end.y));

            let rect = IRect::new(min, max - min);

            if rect.contains(pos) {
                return Some(CableIntersection::OnSegment(id));
            }
        }

        None
    }

    pub fn subdivide_at_intersection(&mut self, int: CableIntersection, pos: IVec2) -> usize {
        match int {
            CableIntersection::OnSegment(i) => {
                self.points.insert(i + 1, pos);
                i + 1
            }
            CableIntersection::OnPoint(i) => i,
        }
    }
}
