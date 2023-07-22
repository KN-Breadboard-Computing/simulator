use components::Component;
use emulator_core::graph::{Graph as InnerGraph, node::UntypedNodeHandle as InnerNodeHandle};
use wasm_bindgen::prelude::*;

pub mod components;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Graph(InnerGraph);

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct NodeHandle(InnerNodeHandle);

#[wasm_bindgen]
impl Graph {
    pub fn new() -> Graph {
        Graph(InnerGraph::new())
    }

    pub fn add_comp(&mut self, component: Component, value: JsValue) -> NodeHandle {
        NodeHandle(component.add_to_graph(&mut self.0, value))
    }

    pub fn get_comp(&mut self, component: Component, node: &NodeHandle) -> JsValue {
        component.get_from_graph(&mut self.0, node.0)
    }

    pub fn update_comp(&mut self, component: Component, node: &NodeHandle, value: JsValue) {
        component.update_in_graph(&mut self.0, node.0, value)
    }

    pub fn add_conn(&mut self, start: &NodeHandle, start_slot: usize, target: &NodeHandle, target_slot: usize) {
        self.0.add_connection(start.0, start_slot, target.0, target_slot)
    }

    pub fn propagate(&mut self, start: &NodeHandle) {
        self.0.propagate_from(start.0)
    }
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}
