use crate::components::Component;

use super::id::NodeId;

#[derive(Debug)]
pub struct Node {
    pub component: Component,
    pub input_slots: Vec<Option<Slot>>,
    pub output_slots: Vec<Option<Slot>>,
}

#[derive(Debug, Clone)]
pub struct Slot {
    pub target_node: NodeId,
    pub target_slot: usize,
}
