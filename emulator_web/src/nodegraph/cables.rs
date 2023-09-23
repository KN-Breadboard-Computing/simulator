use std::collections::HashSet;

use egui::Pos2;
use log::info;
use slotmap::{new_key_type, SlotMap};


#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Cable {
    pub points: Vec<Pos2>,
    pub neighbours: [HashSet<CableId>; 2],
}

#[derive(Debug)]
pub enum CableEnd {
    First,
    Last,
}

#[derive(Debug, Clone, Copy)]
pub enum CableFindResult {
    OnSegment(usize),
    Point(usize),
}

impl Cable {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            neighbours: [HashSet::new(), HashSet::new()],
        }
    }

    pub fn starting_in(point: Pos2) -> Self {
        let mut new = Self::new();
        new.points = vec![point, point];
        new
    }

    pub fn point_at_end(&self, end: CableEnd) {

    }

    pub fn find_point(&self, point: Pos2) -> Option<CableFindResult> {
        if Some(&point) == self.points.first() {
            return Some(CableFindResult::Point(0));
        }

        for (id, window) in self.points.windows(2).enumerate() {
            let &[start, end] = window else { unreachable!() };

            if point == end {
                return Some(CableFindResult::Point(id + 1));
            }

            let x_range = f32::min(start.x, end.x)..=f32::max(start.x, end.x);
            let y_range = f32::min(start.y, end.y)..=f32::max(start.y, end.y);

            if x_range.contains(&point.x) && y_range.contains(&point.y) {
                return Some(CableFindResult::OnSegment(id));
            }
        }

        None
    }

    pub fn move_point_aligned(&mut self, id: &mut usize, new_pos: Pos2) {
        self.points[*id] = new_pos;

        for neigh_offset in [-1, 1] {
            let neigh_id = *id as i32 + neigh_offset;
            if neigh_id < 0 || neigh_id >= self.points.len() as i32 {
                continue;
            }
            let mut neigh_id = neigh_id as usize;

            if neigh_id == 0 {
                self.points.insert(1, self.points[0]);
                neigh_id += 1;
                *id += 1;
            }

            if neigh_id == self.points.len() - 1 {
                self.points
                    .insert(self.points.len() - 2, self.points[self.points.len() - 1]);
            }

            let neigh_neigh_id = (neigh_id as i32 + neigh_offset) as usize;

            let point = self.points[*id];
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
    }

    pub fn not_a_line(&self) -> bool {
        self.points.len() < 2
    }
}

new_key_type! {pub struct CableId;}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CablesGraph {
    pub cables: SlotMap<CableId, Cable>,
}

impl CablesGraph {
    pub fn new() -> Self {
        Self {
            cables: SlotMap::with_key(),
        }
    }

    pub fn find_all_point(
        &self,
        point: Pos2,
    ) -> impl Iterator<Item = (CableId, CableFindResult)> + '_ {
        self.cables
            .iter()
            .filter_map(move |(id, c)| c.find_point(point).map(|r| (id, r)))
    }

    ///TODO 
    ///Crash, kiedy jest pętla i spróbujemy ją przesunąć
    pub fn update_neighbours(&mut self, id: CableId) {
        if self.cables[id].not_a_line() {
            self.cables.remove(id);
            return;
        }

        let old_neighbours_first = &self.cables[id].neighbours[0].clone();
        let old_neighbours_second = &self.cables[id].neighbours[1].clone();
        for &n_id in old_neighbours_first.iter().chain(old_neighbours_second.iter()) {
            self.cables[n_id].neighbours[0].remove(&n_id);
            self.cables[n_id].neighbours[1].remove(&n_id);
        }

        let start = *self.cables[id].points.first().unwrap(); 
        let other_end = *self.cables[id].points.last().unwrap(); 

        let mut update_neighbour = |end: Pos2, n_number: usize| {
            let new_affected = self
                .find_all_point(end)
                .filter(|&(i,_)| i != id)
                .collect::<Vec<_>>();
    
            let mut new_neighbours = HashSet::new();
    
            for (n_id,res) in new_affected {
                let p = match res {
                    CableFindResult::OnSegment(seg) => {
                        self.cables[n_id].points.insert(seg + 1, end);
                        seg + 1
                    },
                    CableFindResult::Point(i) => i,
                };
                if p != 0 && p != self.cables[n_id].points.len() - 1 {
                    let [new_first,new_second] = self.split(n_id, p);
                    self.cables[new_first].neighbours[1].insert(id);
                    self.cables[new_second].neighbours[0].insert(id);
                    new_neighbours.extend([new_first,new_second]);
                } else {
                    if p == 0 {
                        self.cables[n_id].neighbours[0].insert(id);
                    } else {
                        self.cables[n_id].neighbours[1].insert(id);
                    }
                    new_neighbours.insert(n_id);
                }
            }
    
            self.cables[id].neighbours[n_number] = new_neighbours;
        };

        update_neighbour(start, 0);
        update_neighbour(other_end, 1);

    }

    pub fn split(&mut self, id: CableId, point_id: usize) -> [CableId; 2] {
        let Cable {
            mut points,
            neighbours: [start_neighbours, end_neighbours],
        } = self.cables.remove(id).unwrap();

        points.insert(point_id, points[point_id]);

        let (first_points, second_points) = points.split_at(point_id + 1);

        let first_cable = Cable {
            points: first_points.to_vec(),
            neighbours: [start_neighbours, HashSet::new()],
        };
        let second_cable = Cable {
            points: second_points.to_vec(),
            neighbours: [HashSet::new(), end_neighbours],
        };

        let first_id = self.cables.insert(first_cable);
        let second_id = self.cables.insert(second_cable);

        self.cables[first_id].neighbours[1].insert(second_id);
        self.cables[second_id].neighbours[0].insert(first_id);

        [first_id, second_id]
    }

    ///TODO 
    pub fn merge(&mut self, id_a: CableId, id_b: CableId) {
        
    }

    pub fn move_cable_point(&mut self, id: CableId, point_id: &mut usize, new_pos: Pos2) {
        self.cables[id].move_point_aligned(point_id, new_pos);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut graph = CablesGraph::new();

        let mut cable_a = Cable::new();
        cable_a.points = vec![Pos2::new(0.0, 0.0), Pos2::new(10.0, 0.0), Pos2::new(10.0, 10.0)];
        
        let mut cable_b = Cable::new();
        cable_b.points = vec![Pos2::new(10.0, 10.0), Pos2::new(20.0, 10.0)];

        let a = graph.cables.insert(cable_a);
        let b = graph.cables.insert(cable_b);

        graph.update_neighbours(b);
        dbg!(&graph.cables.iter());
    }
}
