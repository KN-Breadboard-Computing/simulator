use egui::{vec2, Pos2, Rect, Vec2};
pub use emulator_core::components::Component as LogicalComponent;
use emulator_core::components::ComponentBehaviour;
pub use emulator_core::graph::id::NodeId as ComponentId;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Component {
    pub pos: Pos2,
    pub size: Vec2,
}

impl Component {
    pub fn rect(&self) -> Rect {
        Rect::from_min_size(self.pos, self.size)
    }

    pub fn new(pos: Pos2, size: Vec2) -> Self {
        Self { pos, size }
    }

    pub fn from_logical_component(comp: &LogicalComponent, pos: Pos2) -> Self {
        let input_size = comp.input_size();
        let output_size = comp.output_size();

        let size = match comp {
            LogicalComponent::Not(_)
            | LogicalComponent::Constant(_)
            | LogicalComponent::DebugOutput(_) => vec2(2.0, 2.0),
            LogicalComponent::And(_) | LogicalComponent::Or(_) | LogicalComponent::Xor(_) => {
                vec2(3.0, 3.0)
            }
            _ => vec2(4.0, ((input_size + 1) * (output_size + 1)) as f32),
        };

        Self { pos : pos - size / 2.0, size }
    }

    pub fn contains(&self, pos: Pos2) -> bool {
        Rect::from_min_size(self.pos, self.size).contains(pos)
    }
}
