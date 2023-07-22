use emulator_core::{
    components::{
        gates::{And, Not, Or, Xor},
        simple::{Constant, DebugOutput, Fork},
    },
    graph::{node::UntypedNodeHandle, Graph},
};
use wasm_bindgen::prelude::*;

pub mod gates;
pub mod simple;

#[wasm_bindgen]
pub enum Component {
    And = "and",
    Or = "or",
    Xor = "xor",
    Not = "not",
    Constant = "const",
    DebugOutput = "out",
    Fork = "fork",
}

impl Component {
    pub fn add_to_graph(self, graph: &mut Graph, val: JsValue) -> UntypedNodeHandle {
        match self {
            Component::And => graph.add_component(And::from_js(val)).into(),
            Component::Or => graph.add_component(Or::from_js(val)).into(),
            Component::Xor => graph.add_component(Xor::from_js(val)).into(),
            Component::Not => graph.add_component(Not::from_js(val)).into(),
            Component::Constant => graph.add_component(Constant::from_js(val)).into(),
            Component::DebugOutput => graph.add_component(DebugOutput::from_js(val)).into(),
            Component::Fork => graph.add_component(Fork::from_js(val)).into(),
            Component::__Nonexhaustive => todo!(),
        }
    }

    pub fn update_in_graph(self, graph: &mut Graph, node: UntypedNodeHandle, val: JsValue) {
        match self {
            Component::And => graph.update_component(node, And::from_js(val)),
            Component::Or => graph.update_component(node, Or::from_js(val)),
            Component::Xor => graph.update_component(node, Xor::from_js(val)),
            Component::Not => graph.update_component(node, Not::from_js(val)),
            Component::Constant => graph.update_component(node, Constant::from_js(val)),
            Component::DebugOutput => graph.update_component(node, DebugOutput::from_js(val)),
            Component::Fork => graph.update_component(node, Fork::from_js(val)),
            Component::__Nonexhaustive => (),
        };
    }

    pub fn get_from_graph(self, graph: &mut Graph, node: UntypedNodeHandle) -> JsValue {
        match self {
            Component::And => graph.get_comp::<And>(node).to_js(),
            Component::Or => graph.get_comp::<Or>(node).to_js(),
            Component::Xor => graph.get_comp::<Xor>(node).to_js(),
            Component::Not => graph.get_comp::<Not>(node).to_js(),
            Component::Constant => graph.get_comp::<Constant>(node).to_js(),
            Component::DebugOutput => graph.get_comp::<DebugOutput>(node).to_js(),
            Component::Fork => graph.get_comp::<Fork>(node).to_js(),
            Component::__Nonexhaustive => JsValue::NULL,
        }
    }
}

pub trait JsComponent {
    fn to_js(&self) -> JsValue;
    fn from_js(val: JsValue) -> Self;
}
