use emulator_core::{graph::{Graph as InnerGraph, id::NodeId as InnerNodeId}, components::Component};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Graph(InnerGraph);

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct NodeId(InnerNodeId);

#[wasm_bindgen]
impl Graph {
    pub fn new() -> Graph {
        Graph(InnerGraph::new())
    }

    pub fn add_comp(&mut self, component: JsValue) -> NodeId {
        let component : Component = from_value(component).unwrap();
        let id = self.0.add_comp(component);
        NodeId(id.into())
    }

    pub fn get_comp(&self, node: &NodeId) -> JsValue {
        let comp = &self.0[node.0];
        to_value(comp).unwrap()
    }

    pub fn set_comp(&mut self, node: &NodeId, value: JsValue) {
        let value : Component = from_value(value).unwrap();
        assert_eq!(std::mem::discriminant(&value), std::mem::discriminant(&self.0[node.0]));
        self.0[node.0] = value;
    }

    pub fn remove_comp(&mut self, node: &NodeId) {
        self.0.remove_comp(node.0)
    }

    pub fn add_conn(&mut self, start: &NodeId, start_slot: usize, target: &NodeId, target_slot: usize) {
        self.0.add_conn(start.0, start_slot, target.0, target_slot)
    }

    pub fn remove_conn(&mut self, start: &NodeId, start_slot: usize, target: &NodeId, target_slot: usize) {
        self.0.remove_conn(start.0, start_slot, target.0, target_slot)
    }

    pub fn propagate(&mut self, start: &NodeId) {
        self.0.propagate_from(start.0)
    }

    pub fn output_state(&self, node: &NodeId) -> usize {
        self.0.outputs[node.0].as_raw_slice()[0]
    }

    pub fn input_state(&self, node: &NodeId) -> usize {
        self.0.inputs[node.0].as_raw_slice()[0]
    }
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use emulator_core::components::{gates::And, Component, simple::Constant};

    #[test]
    fn test() {
        let a : Component = Constant {state: true}.into();
        let x = serde_json::to_string(&a).unwrap();
        println!("{}", x);
    }
}
