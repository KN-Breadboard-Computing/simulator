use serde::{Deserialize, Serialize};

use crate::components::Component;

use super::id::ComponentId;

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    pub component: Component,
    pub input_slots: Vec<Option<Slot>>,
    pub output_slots: Vec<Option<Slot>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Slot {
    pub target_node: ComponentId,
    pub target_slot: usize,
}
