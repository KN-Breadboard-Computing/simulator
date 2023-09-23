use std::collections::{BTreeSet, HashSet};

use egui::{
    epaint::PathShape, pos2, vec2, Align2, Color32, FontId, Painter, Pos2, Rect, Response, Stroke,
    Ui, Vec2,
};
use emulator_core::components::ComponentBehaviour;

use super::{
    cables::CablesGraph, components::Component, components::LogicalComponent,
    transform::NodeGraphTransform,
};

pub struct GridDrawable;

impl GridDrawable {
    pub fn show(&mut self, ui: &Ui, transform: &NodeGraphTransform) {
        let painter = ui.painter();
        let stroke = Stroke::new(2.0 * transform.bounds.zoom, Color32::from_gray(100));

        let x_start = transform.bounds.x.start().ceil();
        let x_end = transform.bounds.x.end().floor();

        let to_screen = transform.to_screen();

        let mut x = x_start;
        while x <= x_end {
            let start = to_screen * pos2(x, *transform.bounds.y.start());
            let end = to_screen * pos2(x, *transform.bounds.y.end());
            painter.line_segment([start, end], stroke);
            x += 1.0;
        }

        let y_start = transform.bounds.y.start().ceil();
        let y_end = transform.bounds.y.end().floor();

        let mut y = y_start;
        while y <= y_end {
            let start = to_screen * pos2(*transform.bounds.x.start(), y);
            let end = to_screen * pos2(*transform.bounds.x.end(), y);
            painter.line_segment([start, end], stroke);
            y += 1.0;
        }
    }
}

pub struct CablesDrawable<'a> {
    graph: &'a CablesGraph,
}

impl<'a> CablesDrawable<'a> {
    pub fn new(graph: &'a CablesGraph) -> Self {
        Self { graph }
    }

    pub fn show(&mut self, ui: &Ui, transform: &NodeGraphTransform) {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        struct Pos2Wrapper(i32, i32);

        let painter = ui.painter_at(transform.screen_frame);

        let stroke = Stroke::new(5.0 * transform.bounds.zoom, Color32::WHITE);
        let to_screen = transform.to_screen();

        let mut splits = HashSet::new();

        for cable in self.graph.map.values().filter(|c| !c.not_a_line()) {
            let points = cable.points.iter().map(|&p| to_screen * p).collect();

            if !cable.neighbours[0].is_empty() {
                let &first = cable.points.first().unwrap();
                splits.insert(Pos2Wrapper(first.x as i32, first.y as i32));
            }

            if !cable.neighbours[1].is_empty() {
                let &last = cable.points.last().unwrap();
                splits.insert(Pos2Wrapper(last.x as i32, last.y as i32));
            }

            painter.add(PathShape::line(points, stroke));
        }

        for Pos2Wrapper(x, y) in splits {
            let pos = pos2(x as f32, y as f32);
            painter.circle_filled(
                to_screen * pos,
                8.0 * transform.bounds.zoom,
                Color32::from_gray(165),
            );
        }
    }
}

pub struct ComponentDrawable<'a> {
    component_data: &'a Component,
    logical_component: &'a LogicalComponent,
}

impl<'a> ComponentDrawable<'a> {
    pub fn new(component: &'a Component, logical: &'a LogicalComponent) -> Self {
        Self {
            component_data: component,
            logical_component: logical,
        }
    }

    pub fn show(&mut self, ui: &Ui, transform: &NodeGraphTransform) {
        let painter = ui.painter_at(transform.screen_frame);
        let to_screen = transform.to_screen();

        let pos = self.component_data.pos;
        let size = self.component_data.size;

        match self.logical_component {
            LogicalComponent::And(_) => draw_gate(&painter, transform, pos, size, &AND_POINTS),
            LogicalComponent::Or(_) => draw_gate(&painter, transform, pos, size, &OR_POINTS),
            LogicalComponent::Xor(_) => draw_gate(&painter, transform, pos, size, &OR_POINTS),
            LogicalComponent::Not(_) => draw_gate(&painter, transform, pos, size, &NOT_POINTS),
            LogicalComponent::Fork(_) => return,
            LogicalComponent::DebugOutput(s) => {
                draw_constant(&painter, transform, pos, size, s.state as u32)
            }
            LogicalComponent::Constant(s) => {
                draw_constant(&painter, transform, pos, size, s.state as u32)
            }
        }

        draw_slots(
            &painter,
            transform,
            pos,
            size,
            self.logical_component.input_size(),
            self.logical_component.output_size(),
        );
    }
}

fn draw_slots(
    painter: &Painter,
    transform: &NodeGraphTransform,
    pos: Pos2,
    size: Vec2,
    inputs: usize,
    outputs: usize,
) {
    let inputs_gap = size.y as usize / (inputs + 1);
    let outputs_gap = size.y as usize / (outputs + 1);

    let to_screen = transform.to_screen();

    for i in 1..=inputs {
        let pos = to_screen * (pos + vec2(0.0, (i * inputs_gap) as f32));
        painter.circle_filled(pos, 8.0 * transform.bounds.zoom, Color32::RED);
    }

    for i in 1..=outputs {
        let pos = to_screen * (pos + vec2(size.x, (i * outputs_gap) as f32));
        painter.circle_filled(pos, 8.0 * transform.bounds.zoom, Color32::GREEN);
    }
}

const NOT_POINTS: [Vec2; 3] = [vec2(0.0, 0.0), vec2(2.0, 1.0), vec2(0.0, 2.0)];

const AND_POINTS: [Vec2; 11] = [
    vec2(0.0, 0.0),
    vec2(1.0, 0.0),
    vec2(1.2, 0.03),
    vec2(1.6, 0.2),
    vec2(1.8, 0.4),
    vec2(2.0, 1.0),
    vec2(1.8, 1.6),
    vec2(1.6, 1.8),
    vec2(1.2, 1.97),
    vec2(1.0, 2.0),
    vec2(0.0, 2.0),
];

const OR_POINTS: [Vec2; 11] = [
    vec2(-0.25, 0.0),
    vec2(1.0, 0.0),
    vec2(1.4, 0.2),
    vec2(1.8, 0.6),
    vec2(2.0, 1.0),
    vec2(1.8, 1.4),
    vec2(1.4, 1.8),
    vec2(1.0, 2.0),
    vec2(-0.25, 2.0),
    vec2(0.0, 1.5),
    vec2(0.0, 0.5),
];

fn draw_gate(
    painter: &Painter,
    transform: &NodeGraphTransform,
    pos: Pos2,
    size: Vec2,
    points: &[Vec2],
) {
    let to_screen = transform.to_screen();
    let stroke = Stroke::new(5.0 * transform.bounds.zoom, Color32::WHITE);

    let points = points
        .iter()
        .map(|&p| to_screen * (pos + p * size / vec2(2.0, 2.0)))
        .collect();

    painter.add(PathShape::convex_polygon(points, Color32::BLACK, stroke));
}

fn draw_constant(
    painter: &Painter,
    transform: &NodeGraphTransform,
    pos: Pos2,
    size: Vec2,
    state: u32,
) {
    let to_screen = transform.to_screen();
    let stroke = Stroke::new(5.0 * transform.bounds.zoom, Color32::WHITE);

    painter.rect(
        Rect::from_min_max(to_screen * pos, to_screen * (pos + size)),
        0.0,
        Color32::BLACK,
        stroke,
    );
    painter.text(
        to_screen * (pos + size / 2.0),
        Align2::CENTER_CENTER,
        format!("{state}"),
        FontId::default(),
        Color32::WHITE,
    );
}
