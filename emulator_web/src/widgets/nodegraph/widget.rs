use std::collections::HashSet;

use egui::{
    epaint::PathShape, pos2, vec2, Align2, Color32, FontId, Painter, Pos2, Rect, Response, Sense,
    Stroke, Ui, Vec2,
};
use emulator_core::components::Component;
use log::info;

use crate::{
    app,
    components::registry::ComponentRegistry,
    nodegraph::{
        cables::Cable,
        components::ComponentIntersection,
        graph::NodeGraph,
        transform::{NodeGraphBounds, NodeGraphTransform},
    },
    state::{
        app::AppState,
        modes::{AddingOptions, Mode, ModeState},
        selection::{Selection, SelectionAction, SelectionState},
    },
    util::{IRect, IVec2},
};

pub fn nodegraph_widget(ui: &mut Ui, app_state: &mut AppState) {
    let size = ui.available_size();
    let (response, painter) = ui.allocate_painter(size, Sense::click_and_drag());
    let transform = NodeGraphTransform::new(response.rect, app_state.node_graph.bounds.clone());

    let click_pos = response
        .interact_pointer_pos()
        .filter(|_| response.clicked())
        .map(|pos| transform.point_to_bounds(pos));
    let hover_pos = response
        .hover_pos()
        .map(|pos| transform.point_to_bounds(pos));

    reset_highlights(&mut app_state.node_graph);

    match (app_state.selection_state.action, click_pos, hover_pos) {
        (SelectionAction::Nothing, Some(pos), _) => {
            use_mode_click_handling(app_state, pos);
        }
        (SelectionAction::Nothing, _, Some(pos)) => {
            highlight_hovered(&mut app_state.node_graph, pos);
        }
        (SelectionAction::Moving, None, Some(pos)) => {
            move_selection(
                &mut app_state.node_graph,
                &mut app_state.selection_state.selection,
                pos,
            );
        }
        (SelectionAction::Moving, Some(_), _) => {
            deselect_all(&mut app_state.node_graph, &mut app_state.selection_state);
        }
        _ => {}
    }

    grid_controls(ui, &response, &mut app_state.node_graph.bounds, &transform);
    grid(&painter, &transform);

    comps(&painter, &mut app_state.node_graph, &transform);
    cables(&painter, &mut app_state.node_graph, &transform)
}

fn use_mode_click_handling(app_state: &mut AppState, pos: Pos2) {
    let AppState {
        mode_state,
        selection_state,
        node_graph,
        registry,
    } = app_state;

    match mode_state.mode {
        Mode::Running => {
            comps_clicked_controls(&mut app_state.node_graph, pos);
            output_cables_coloring(&mut app_state.node_graph);
        }
        Mode::Adding => match mode_state.add_opt {
            AddingOptions::Cable => {
                let id = node_graph.new_cable(pos.into());
                selection_state.selection.cables.push((id, 1));
                selection_state.action = SelectionAction::Moving;
            }
            AddingOptions::ComponentRid(rid) => {
                let id = node_graph.new_component(pos.into(), rid, registry);
                selection_state.selection.comps.push(id);
                selection_state.action = SelectionAction::Moving;
            }
        },
        Mode::Editing => {
            select_clicked(node_graph, selection_state, pos);
            selection_state.action = SelectionAction::Moving;
        }
        Mode::Deleting => {
            let hit_cables = node_graph.cables_intersecting(pos, 0.2).collect::<Vec<_>>();
            for (cable_id, _) in hit_cables {
                node_graph.remove_cable_and_fix(cable_id);
            }

            let hit_comps = node_graph
                .components_intersecting(pos, 0.2)
                .collect::<Vec<_>>();
            for (comp_id, _) in hit_comps {
                node_graph.remove_comp(comp_id);
            }
        }
    }
}

pub fn select_clicked(
    nodegraph: &mut NodeGraph,
    selection_state: &mut SelectionState,
    clicked_pos: Pos2,
) {
    let hit_cables = nodegraph
        .cables_intersecting(clicked_pos, 0.2)
        .collect::<Vec<_>>()
        .into_iter()
        .map(|(id, int)| {
            (
                id,
                nodegraph
                    .cable_mut(id)
                    .subdivide_at_intersection(int, clicked_pos.into()),
            )
        })
        .collect();
    selection_state.selection.cables = hit_cables;

    let hit_comps = nodegraph
        .components_intersecting(clicked_pos, 0.2)
        .filter_map(|(id, int)| {
            if int == ComponentIntersection::Inside {
                Some(id)
            } else {
                None
            }
        })
        .collect();
    selection_state.selection.comps = hit_comps;
}

fn deselect_all(nodegraph: &mut NodeGraph, selection_state: &mut SelectionState) {
    for &(cable_id, point_id) in &selection_state.selection.cables {
        if point_id == nodegraph.cable(cable_id).points.len() - 1 || point_id == 0 {
            nodegraph.fix_after_moving_cable(cable_id);
        }
        nodegraph.cable_mut(cable_id).clean_flat_points();
    }

    selection_state.selection = Selection::default();
    selection_state.action = SelectionAction::Nothing;
}

fn move_selection(nodegraph: &mut NodeGraph, selection: &mut Selection, pos: Pos2) {
    for (cable_id, point_id) in &mut selection.cables {
        *point_id = nodegraph
            .cable_mut(*cable_id)
            .move_point_aligned(*point_id, pos.into());
    }

    for &comp_id in &selection.comps {
        nodegraph.comp_mut(comp_id).move_middle(pos.into());
    }
}

fn grid(painter: &Painter, transform: &NodeGraphTransform) {
    let stroke = Stroke::new(1.5 * transform.bounds.zoom, Color32::from_gray(75));

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

fn grid_controls(
    ui: &Ui,
    response: &Response,
    bounds: &mut NodeGraphBounds,
    transform: &NodeGraphTransform,
) {
    if response.dragged() {
        let delta = response.drag_delta();
        bounds.translate(-transform.to_bounds().scale() * delta);
    }

    let scroll = ui.input(|i| i.zoom_delta());
    bounds.zoom_in(scroll);
}

pub fn reset_highlights(nodegraph: &mut NodeGraph) {
    for cable in nodegraph.cables.values_mut() {
        cable.highlight_level = 0;
    }

    for comp in nodegraph.components.values_mut() {
        comp.highlight_level = 0;
    }
}

pub fn highlight_hovered(nodegraph: &mut NodeGraph, pos: Pos2) {
    let hovered = nodegraph.cables_intersecting(pos, 0.2).collect::<Vec<_>>();

    for &(cable_id, _) in &hovered {
        nodegraph.cable_mut(cable_id).highlight_level =
            nodegraph.cable_mut(cable_id).highlight_level.max(2);
        for &n in &nodegraph.cable(cable_id).neighbours[0].clone() {
            nodegraph.cable_mut(n).highlight_level = nodegraph.cable(n).highlight_level.max(1);
        }
        for &n in &nodegraph.cable(cable_id).neighbours[1].clone() {
            nodegraph.cable_mut(n).highlight_level = nodegraph.cable(n).highlight_level.max(1);
        }
        for &c in nodegraph.cable(cable_id).conn_comp.clone().iter().flatten() {
            nodegraph.comp_mut(c).highlight_level = nodegraph.comp(c).highlight_level.max(1);
        }
    }

    let hovered = nodegraph
        .components_intersecting(pos, 0.2)
        .filter(|(_, int)| int == &ComponentIntersection::Inside)
        .collect::<Vec<_>>();

    for &(comp_id, _) in &hovered {
        nodegraph.comp_mut(comp_id).highlight_level =
            nodegraph.comp(comp_id).highlight_level.max(2);
        for &n in nodegraph
            .comp(comp_id)
            .input_cables
            .clone()
            .iter()
            .flatten()
        {
            nodegraph.cable_mut(n).highlight_level = nodegraph.cable(n).highlight_level.max(1);
        }
        for &n in nodegraph
            .comp(comp_id)
            .output_cables
            .clone()
            .iter()
            .flatten()
        {
            nodegraph.cable_mut(n).highlight_level = nodegraph.cable(n).highlight_level.max(1);
        }
    }
}

pub fn output_cables_coloring(nodegraph: &mut NodeGraph) {
    for c_id in nodegraph.components.keys() {
        let output = &nodegraph.graph.outputs[c_id];
        for (i, maybe_output_cable_id) in
            nodegraph.components[c_id].output_cables.iter().enumerate()
        {
            if let &Some(output_cable_id) = maybe_output_cable_id {
                let group = nodegraph.travel_cable_group(output_cable_id);
                let color = if output[i] {
                    Cable::ACTIVATED_COLOR
                } else {
                    Cable::DEFAULT_COLOR
                };
                for &group_cable_id in &group {
                    nodegraph.cables[group_cable_id].color = color;
                }
            }
        }
    }
}

pub fn cables(painter: &Painter, nodegraph: &NodeGraph, transform: &NodeGraphTransform) {
    let stroke = |color: Color32| Stroke::new(5.0 * transform.bounds.zoom, color);

    let mut splits = HashSet::new();

    for cable in nodegraph.cables.values().filter(|c| !c.not_a_line()) {
        let points: Vec<_> = cable
            .points
            .iter()
            .map(|&p| transform.point_i_to_screen(p))
            .collect();

        if !cable.neighbours[0].is_empty() {
            let &first = cable.points.first().unwrap();
            splits.insert(first);
        }

        if !cable.neighbours[1].is_empty() {
            let &last = cable.points.last().unwrap();
            splits.insert(last);
        }

        if cable.highlight_level == 1 {
            painter.add(PathShape::line(
                points.clone(),
                Stroke::new(15.0 * transform.bounds.zoom, Color32::DARK_GREEN),
            ));
        } else if cable.highlight_level == 2 {
            painter.add(PathShape::line(
                points.clone(),
                Stroke::new(15.0 * transform.bounds.zoom, Color32::BLUE),
            ));
        }
        painter.add(PathShape::line(points, stroke(cable.color)));
    }

    for split_pos in splits {
        let pos = transform.point_i_to_screen(split_pos);
        painter.circle_filled(pos, 10.0 * transform.bounds.zoom, Color32::from_gray(150));
    }
}

pub fn comps(painter: &Painter, nodegraph: &NodeGraph, transform: &NodeGraphTransform) {
    for (id, comp) in nodegraph.components.iter() {
        let rect = Rect::from_min_max(
            comp.rect.pos.into(),
            (comp.rect.pos + comp.rect.size).into(),
        );
        if nodegraph.comp(id).highlight_level == 2 {
            painter.rect_filled(
                Rect::from_min_max(
                    transform.point_to_screen(rect.min - vec2(0.5, 0.5)),
                    transform.point_to_screen(rect.max + vec2(0.5, 0.5)),
                ),
                5.0,
                Color32::BLUE,
            );
        } else if nodegraph.comp(id).highlight_level == 1 {
            painter.rect_filled(
                Rect::from_min_max(
                    transform.point_to_screen(rect.min - vec2(0.5, 0.5)),
                    transform.point_to_screen(rect.max + vec2(0.5, 0.5)),
                ),
                5.0,
                Color32::DARK_GREEN,
            );
        }

        match nodegraph.graph[id] {
            Component::And(_) => draw_gate(painter, transform, comp.rect, &AND_POINTS),
            Component::Or(_) => draw_gate(painter, transform, comp.rect, &OR_POINTS),
            Component::Xor(_) => draw_gate(painter, transform, comp.rect, &OR_POINTS),
            Component::Not(_) => draw_gate(painter, transform, comp.rect, &NOT_POINTS),
            Component::Fork(_) => return,
            Component::DebugOutput(s) => {
                draw_constant(painter, transform, comp.rect, s.state as u32)
            }
            Component::Constant(s) => draw_constant(painter, transform, comp.rect, s.state as u32),
        }

        draw_slots(
            painter,
            transform,
            comp.rect.pos,
            &comp.input_slots,
            &comp.output_slots,
        );
    }
}

pub fn comps_clicked_controls(nodegraph: &mut NodeGraph, clicked_pos: Pos2) {
    let comps = nodegraph
        .components_intersecting(clicked_pos, 0.2)
        .map(|(id, _)| id)
        .collect::<Vec<_>>();

    for c_id in comps {
        let comp = &mut nodegraph.graph[c_id];

        if let Component::Constant(x) = comp {
            x.state = !x.state;
        }

        nodegraph.graph.propagate_from(c_id);
    }
}

fn draw_slots(
    painter: &Painter,
    transform: &NodeGraphTransform,
    origin: IVec2,
    input_slots: &[IVec2],
    output_slots: &[IVec2],
) {
    for &input in input_slots {
        painter.circle_filled(
            transform.point_i_to_screen(origin + input),
            8.0 * transform.bounds.zoom,
            Color32::RED,
        );
    }

    for &output in output_slots {
        painter.circle_filled(
            transform.point_i_to_screen(origin + output),
            8.0 * transform.bounds.zoom,
            Color32::GREEN,
        );
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
    vec2(0.0, 0.0),
    vec2(1.0, 0.0),
    vec2(1.4, 0.2),
    vec2(1.8, 0.6),
    vec2(2.0, 1.0),
    vec2(1.8, 1.4),
    vec2(1.4, 1.8),
    vec2(1.0, 2.0),
    vec2(0.0, 2.0),
    vec2(0.1, 1.5),
    vec2(0.1, 0.5),
];

fn draw_gate(painter: &Painter, transform: &NodeGraphTransform, rect: IRect, points: &[Vec2]) {
    let stroke = Stroke::new(5.0 * transform.bounds.zoom, Color32::WHITE);

    let pos: Pos2 = rect.pos.into();
    let size: Vec2 = rect.size.into();

    let points = points
        .iter()
        .map(|&p| transform.point_to_screen(pos + p * size / vec2(2.0, 2.0)))
        .collect();

    painter.add(PathShape::convex_polygon(points, Color32::BLACK, stroke));
}

fn draw_constant(painter: &Painter, transform: &NodeGraphTransform, rect: IRect, state: u32) {
    let stroke = Stroke::new(5.0 * transform.bounds.zoom, Color32::WHITE);

    painter.rect(
        Rect::from_min_max(
            transform.point_i_to_screen(rect.pos),
            transform.point_i_to_screen(rect.pos + rect.size),
        ),
        0.0,
        Color32::BLACK,
        stroke,
    );

    let pos: Pos2 = rect.pos.into();
    let size: Vec2 = rect.size.into();
    painter.text(
        transform.point_to_screen(pos + size / 2.0),
        Align2::CENTER_CENTER,
        format!("{state}"),
        FontId::default(),
        Color32::WHITE,
    );
}
