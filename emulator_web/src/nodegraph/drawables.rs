use std::collections::{BTreeSet, HashSet};

use egui::{epaint::PathShape, pos2, vec2, Color32, Pos2, Response, Stroke, Ui};

use super::{cables::CablesGraph, components::Component, transform::NodeGraphTransform};

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

        for cable in self.graph.cables.values().filter(|c| !c.not_a_line()) {
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
    component: &'a Component,
}

impl<'a> ComponentDrawable<'a> {
    pub fn new(component: &'a Component) -> Self {
        Self { component }
    }

    pub fn show(&mut self, ui: &Ui, transform: &NodeGraphTransform) {
        let painter = ui.painter_at(transform.screen_frame);
        let to_screen = transform.to_screen();
        let stroke = Stroke::new(5.0 * transform.bounds.zoom, Color32::WHITE);

        let pos = self.component.pos;

        painter.add(PathShape::convex_polygon(
            vec![
                to_screen * pos,
                to_screen * (pos + vec2(2.0, 1.0)),
                to_screen * (pos + vec2(0.0, 2.0)),
            ],
            Color32::BLACK,
            stroke,
        ));

        painter.circle_filled(to_screen * (pos + vec2(0.0, 1.0)), 8.0 * transform.bounds.zoom, Color32::RED);
        painter.circle_filled(to_screen * (pos + vec2(2.0, 1.0)), 8.0 * transform.bounds.zoom, Color32::GREEN);
    }
}
