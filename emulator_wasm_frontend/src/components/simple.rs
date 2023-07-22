use emulator_core::components::simple::{Constant, DebugOutput, Fork};
use wasm_bindgen::JsValue;

use super::JsComponent;

impl JsComponent for Constant {
    fn to_js(&self) -> wasm_bindgen::JsValue {
        self.state.into()
    }

    fn from_js(val: wasm_bindgen::JsValue) -> Self {
        Self { state: val.as_bool().unwrap() }
    }
}

impl JsComponent for DebugOutput {
    fn to_js(&self) -> wasm_bindgen::JsValue {
        self.state.into()
    }

    fn from_js(val: wasm_bindgen::JsValue) -> Self {
        Self { state: val.as_bool().unwrap() }
    }
}

impl JsComponent for Fork {
    fn to_js(&self) -> wasm_bindgen::JsValue {
        JsValue::NULL
    }

    fn from_js(val: wasm_bindgen::JsValue) -> Self {
        Self
    }
}