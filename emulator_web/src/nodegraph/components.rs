use egui::{Pos2, Vec2, Rect};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Component {
    pub pos: Pos2,
    pub size: Vec2,
}

impl Component {
    pub fn rect(&self) -> Rect {
        Rect::from_min_size(self.pos, self.size)
    }
}