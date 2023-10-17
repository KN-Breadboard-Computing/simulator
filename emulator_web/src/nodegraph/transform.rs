use std::ops::RangeInclusive;

use egui::{Vec2, Rect, emath::RectTransform, Pos2};

use crate::util::IVec2;

#[derive(Debug,Clone,serde::Deserialize,serde::Serialize)]
pub struct NodeGraphBounds {
    pub x: RangeInclusive<f32>,
    pub y: RangeInclusive<f32>,
    pub zoom: f32,
}

impl Default for NodeGraphBounds {
    fn default() -> Self {
        Self { x: -20.0..=20.0, y: -20.0..=20.0, zoom: 1.0 }
    }
}

impl NodeGraphBounds {
    pub fn translate(&mut self, vec: Vec2) {
        self.x = (self.x.start() + vec.x)..=(self.x.end() + vec.x);
        self.y = (self.y.start() + vec.y)..=(self.y.end() + vec.y);
    }

    pub fn zoom_in(&mut self, factor: f32) {
        self.zoom *= factor;
        self.x = (self.x.start() / factor)..=(self.x.end() / factor);
        self.y = (self.y.start() / factor)..=(self.y.end() / factor);
    }

    pub fn rect(&self) -> Rect {
        Rect::from_x_y_ranges(self.x.clone(), self.y.clone())
    }
}

#[derive(Debug, Clone)]
pub struct NodeGraphTransform {
    pub screen_frame: Rect,
    pub bounds: NodeGraphBounds,
}

impl NodeGraphTransform {
    pub fn new(screen_frame : Rect, bounds: NodeGraphBounds) -> NodeGraphTransform {
        Self { screen_frame, bounds }
    }

    pub fn to_screen(&self) -> RectTransform {
        RectTransform::from_to(self.bounds.rect(), self.screen_frame)
    }

    pub fn to_bounds(&self) -> RectTransform {
        RectTransform::from_to(self.screen_frame, self.bounds.rect())
    }

    pub fn point_to_screen(&self, point: Pos2) -> Pos2 {
        self.to_screen().transform_pos(point)
    }

    pub fn point_to_bounds(&self, point: Pos2) -> Pos2 {
        self.to_bounds().transform_pos(point)
    }

    pub fn point_i_to_screen(&self, point: IVec2) -> Pos2 {
        self.point_to_screen(point.into())
    }

    pub fn point_to_bounds_i(&self, point: Pos2) -> IVec2 {
        self.point_to_bounds(point).into()
    }

}