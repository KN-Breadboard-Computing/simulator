use emulator_core::*;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Graph(graph::Graph);

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct NodeHandle(graph::node::UntypedNodeHandle);

#[wasm_bindgen]
impl Graph {

}
