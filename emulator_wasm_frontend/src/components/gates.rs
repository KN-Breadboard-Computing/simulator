use emulator_core::components::gates::{And, Or, Xor, Not};
use wasm_bindgen::JsValue;

use super::JsComponent;

impl JsComponent for And {
    fn from_js(value: JsValue) -> Self {
        Self
    }

    fn to_js(&self) -> JsValue {
        JsValue::NULL
    }
}

impl JsComponent for Or {
    fn from_js(value: JsValue) -> Self {
        Self
    }

    fn to_js(&self) -> JsValue {
        JsValue::NULL
    }
}

impl JsComponent for Xor {
    fn from_js(value: JsValue) -> Self {
        Self
    }

    fn to_js(&self) -> JsValue {
        JsValue::NULL
    }
}

impl JsComponent for Not {
    fn from_js(value: JsValue) -> Self {
        Self
    }

    fn to_js(&self) -> JsValue {
        JsValue::NULL
    }
}