use emulator_core::components::{Component, ComponentBehaviour};

use crate::util::{IVec2, ivec2};

pub fn get_size(component: &Component) -> IVec2 {
    match component {
        Component::And(_) | Component::Or(_) | Component::Xor(_) | Component::Not(_) => ivec2(4, 6),
        Component::DebugOutput(_) | Component::Constant(_) => ivec2(2, 2),
        _ => {
            let max_slots = component.input_size().max(component.output_size());
            ivec2(6, max_slots as i32 + 1)
        },
    }
}
