use slotmap::new_key_type;

use crate::components::Component;

new_key_type! {pub struct UntypedNodeHandle;}

#[derive(Debug)]
pub struct Node {
    pub component: Box<dyn Component>,
    pub input_slots: Vec<Option<Slot>>,
    pub output_slots: Vec<Option<Slot>>,
}

#[derive(Debug, Clone)]
pub struct Slot {
    pub target_node: UntypedNodeHandle,
    pub target_slot: usize,
}
