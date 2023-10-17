use egui::{Vec2, Pos2};

use crate::{
    components::registry::ComponentRid,
    util::{ivec2, IRect, IVec2},
};

use super::cables::CableId;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComponentNode {
    pub rect: IRect,
    pub input_slots: Vec<IVec2>,
    pub output_slots: Vec<IVec2>,
    pub input_cables: Vec<Option<CableId>>,
    pub output_cables: Vec<Option<CableId>>,
    pub base_entry_rid: ComponentRid,
    pub highlight_level: u8,
}

impl ComponentNode {
    pub fn new(rect: IRect, entry_rid: ComponentRid) -> Self {
        Self {
            rect,
            input_slots: Vec::new(),
            output_slots: Vec::new(),
            input_cables: Vec::new(),
            output_cables: Vec::new(),
            base_entry_rid: entry_rid,
            highlight_level: 0
        }
    }

    pub fn with_default_slots(mut self, input_size: usize, output_size: usize) -> Self {
        let input_gap = self.rect.size.y as usize / (input_size + 1);
        let input_slots =
            (1..=input_size).map(move |i| ivec2(0, (i * input_gap) as i32));
        self.input_slots = input_slots.collect();

        let output_gap = self.rect.size.y as usize / (output_size + 1);
        let output_slots = (1..=output_size)
            .map(move |i| ivec2(self.rect.size.x, (i * output_gap) as i32));
        self.output_slots = output_slots.collect();

        self.input_cables = vec![None; input_size];
        self.output_cables = vec![None; output_size];

        self
    }

    pub fn move_middle(&mut self, new_pos: IVec2) {
        self.rect.pos = new_pos - self.rect.size / 2;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentIntersection {
    InputSlot(usize),
    OutputSlot(usize),
    Inside,
}

impl ComponentNode {
    pub fn intersection_test(&self, pos: Pos2, epsilon: f32) -> Option<ComponentIntersection> {
        for (i, &input) in self.input_slots.iter().enumerate() {
            if (self.rect.pos + input).equals_with_rounding(pos, epsilon) {
                return Some(ComponentIntersection::InputSlot(i));
            }
        }

        for (i, &output) in self.output_slots.iter().enumerate() {
            if (self.rect.pos + output).equals_with_rounding(pos, epsilon) {
                return Some(ComponentIntersection::OutputSlot(i));
            }
        }

        if self.rect.contains_with_rounding(pos, epsilon) {
            return Some(ComponentIntersection::Inside);
        }

        None
    }

    pub fn intersection_test_exact(&self, pos: IVec2) -> Option<ComponentIntersection> {
        for (i, &input) in self.input_slots.iter().enumerate() {
            if self.rect.pos + input == pos {
                return Some(ComponentIntersection::InputSlot(i));
            }
        }

        for (i, &output) in self.output_slots.iter().enumerate() {
            if self.rect.pos + output == pos {
                return Some(ComponentIntersection::OutputSlot(i));
            }
        }

        if self.rect.contains(pos) {
            return Some(ComponentIntersection::Inside);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slots_pos_test() {
        let comp =
            ComponentNode::new(IRect::new(ivec2(0, 0), ivec2(0, 6)), 0).with_default_slots(2, 1);

        assert_eq!(comp.input_slots, vec![ivec2(0, 2), ivec2(0, 4)]);
        assert_eq!(comp.output_slots, vec![ivec2(0, 3)]);
    }
}
