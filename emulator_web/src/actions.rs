use egui::Response;
use emulator_core::components::{
    gates::{And, Not, Or},
    simple::{Constant, DebugOutput},
};

use crate::{
    nodegraph::{
        cables::{Cable, CableFindResult, CableId},
        components::ComponentId,
        components::LogicalComponent,
        transform::NodeGraphTransform,
    },
    EmulatorApp,
};

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub enum ActionState {
    #[default]
    None,
    CableDraw(Vec<(CableId, usize)>),
    ComponentMove(ComponentId),
}

#[derive(Default, PartialEq, serde::Serialize, serde::Deserialize, Clone, Copy)]
pub enum Mode {
    #[default]
    Running,
    CableAdd,
    ComponentAdd,
    Editing,
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, Clone, Copy)]
pub enum ChoosenComponent {
    And,
    Or,
    Not,
    Constant,
    DebugOutput,
}

pub const CHOOSE_LIST: &[(&str, ChoosenComponent)] = &[
    ("And", ChoosenComponent::And),
    ("Or", ChoosenComponent::Or),
    ("Not", ChoosenComponent::Not),
    ("Constant", ChoosenComponent::Constant),
    ("Debug", ChoosenComponent::DebugOutput),
];

pub fn component_from_choosen(choosen: ChoosenComponent) -> LogicalComponent {
    match choosen {
        ChoosenComponent::And => And.into(),
        ChoosenComponent::Or => Or.into(),
        ChoosenComponent::Not => Not.into(),
        ChoosenComponent::Constant => Constant::default().into(),
        ChoosenComponent::DebugOutput => DebugOutput::default().into(),
    }
}

pub fn handle_actions(app_state: &mut EmulatorApp, ui_response: &Response) {
    let transform = NodeGraphTransform::new(ui_response.rect, app_state.bounds.clone());
    let from_screen = transform.to_bounds();

    let interact_grid_pos = ui_response
        .interact_pointer_pos()
        .map(|pos| (from_screen * pos).round());

    let hover_grid_pos = ui_response
        .hover_pos()
        .map(|pos| (from_screen * pos).round());

    match (&mut app_state.action, &app_state.mode) {
        (ActionState::None, Mode::Running) => {
            if ui_response.clicked() {
                let Some(pos) = interact_grid_pos else { return; };
                let touched_component = app_state
                    .graph
                    .components
                    .iter()
                    .find(|(_, c)| c.rect().contains(pos))
                    .map(|(id, _)| id);

                if let Some(component_id) = touched_component {
                    let logical = &mut app_state.graph.logical_graph[component_id];

                    if let LogicalComponent::Constant(s) = logical {
                        s.state = !s.state;
                        app_state.graph.logical_graph.propagate_from(component_id);
                    }
                }
            }
        }
        (ActionState::None, Mode::CableAdd) => {
            if ui_response.clicked() {
                let Some(pos) = interact_grid_pos else { return; };
                let cable = Cable::starting_in(pos);
                let id = app_state.graph.cables.map.insert(cable);
                app_state.action = ActionState::CableDraw(vec![(id, 1)]);
            }
        }
        (ActionState::None, Mode::ComponentAdd) => {
            if ui_response.clicked() {
                let Some(pos) = interact_grid_pos else { return; };
                let component = component_from_choosen(app_state.choosen_component);
                app_state.graph.add_component(component, pos);
            }
        }
        (ActionState::None, Mode::Editing) => {
            if ui_response.clicked() {
                let Some(pos) = interact_grid_pos else { return; };

                let touched_component = app_state
                    .graph
                    .components
                    .iter()
                    .find(|(_, c)| c.rect().contains(pos))
                    .map(|(id, _)| id);

                if let Some(component_id) = touched_component {
                    app_state.action = ActionState::ComponentMove(component_id);
                    return;
                }

                let touched_cables_and_points = app_state
                    .graph
                    .cables
                    .find_all_point(pos)
                    .collect::<Vec<_>>()
                    .into_iter()
                    .map(|(id, result)| match result {
                        CableFindResult::OnSegment(seg) => {
                            app_state.graph.cables.map[id].points.insert(seg + 1, pos);
                            (id, seg + 1)
                        }
                        CableFindResult::Point(point_id) => (id, point_id),
                    })
                    .collect::<Vec<_>>();

                if !touched_cables_and_points.is_empty() {
                    app_state.action = ActionState::CableDraw(touched_cables_and_points);
                }
            }
        }
        (ActionState::CableDraw(edited_cables), _) => {
            if ui_response.clicked() {
                for &(id, _) in edited_cables.iter() {
                    if app_state.graph.cables.map.contains_key(id) {
                        app_state.graph.cables.update_neighbours(id);
                        app_state.graph.connect_cable(id);
                    }
                }
                app_state.action = ActionState::None;
            } else {
                let Some(pos) = hover_grid_pos else {return;};
                for (id, point_id) in edited_cables.iter_mut() {
                    app_state.graph.cables.map[*id].move_point_aligned(point_id, pos);
                }
            }
        }
        (ActionState::ComponentMove(id), _) => {
            if ui_response.clicked() {
                app_state.action = ActionState::None;
            } else {
                let Some(pos) = hover_grid_pos else {return;};
                let component = &mut app_state.graph.components[*id];
                component.pos = (pos - component.size / 2.0).round()
            }
        }
    }
}
