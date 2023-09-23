use egui::{pos2, Color32, Pos2, Sense, vec2};
use log::info;

use crate::nodegraph::{
    cables::{Cable, CableFindResult, CableId, CablesGraph},
    drawables::{CablesDrawable, GridDrawable, ComponentDrawable},
    transform::{NodeGraphBounds, NodeGraphTransform}, components::Component,
};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Emulator {
    bounds: NodeGraphBounds,
    cables: CablesGraph,
    components: Vec<Component>,
    mode: Mode,
    #[serde(skip)]
    action: ActionState,
}

#[derive(Default, serde::Serialize, serde::Deserialize)]
enum ActionState {
    #[default]
    None,
    CableDraw(Vec<(CableId, usize)>),
    ComponentMove(usize)
}

#[derive(Default, PartialEq, serde::Serialize, serde::Deserialize)]
enum Mode {
    #[default]
    OnlyMovement,
    CableEdit,
    ComponentAdd,
}

impl Default for Emulator {
    fn default() -> Self {
        Self {
            bounds: Default::default(),
            cables: CablesGraph::new(),
            action: Default::default(),
            mode: Default::default(),
            components: Default::default(),
        }
    }
}

impl Emulator {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for Emulator {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::new(egui::panel::Side::Left, "Side Panel").show(ctx, |ui| {
            ui.label("Testing");
            if ui.button("Clear").clicked() {
                self.cables = CablesGraph::new();
                self.components = Default::default();
            }
            if ui.button("Reset View").clicked() {
                self.bounds = NodeGraphBounds::default();
            }
            ui.radio_value(&mut self.mode, Mode::OnlyMovement, "Only Movement");
            ui.radio_value(&mut self.mode, Mode::CableEdit, "Cable Edit");
            ui.radio_value(&mut self.mode, Mode::ComponentAdd, "Component Add");
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let size = ui.available_size();
            let response = ui.allocate_response(size, Sense::click_and_drag());

            let transform = NodeGraphTransform::new(response.rect, self.bounds.clone());
            let from_screen = transform.to_bounds();

            match &mut self.action {
                ActionState::None => {
                    if response.clicked() {
                        if let Some(pos) = response.interact_pointer_pos() {
                            let pos = (from_screen * pos).round();

                            match self.mode {
                                Mode::CableEdit => {
                                    let affected =
                                        self.cables.find_all_point(pos).collect::<Vec<_>>();
                                    info!("{:?}", &affected);

                                    if affected.is_empty() {
                                        info!("empty {:?}", self.cables);
                                        let id = self.cables.cables.insert(Cable::starting_in(pos));
                                        self.action = ActionState::CableDraw(vec![(id, 1)]);
                                    } else {
                                        let affected =
                                            affected.into_iter().map(|(id, result)| match result {
                                                CableFindResult::OnSegment(seg) => {
                                                    self.cables.cables[id]
                                                        .points
                                                        .insert(seg + 1, pos);
                                                    (id, seg + 1)
                                                }
                                                CableFindResult::Point(point_id) => (id, point_id),
                                            });
                                        self.action = ActionState::CableDraw(affected.collect());
                                    }
                                }
                                Mode::ComponentAdd => {
                                    let touched = self.components.iter().position(|c| c.rect().contains(pos));

                                    if let Some(touched) = touched {
                                        self.action = ActionState::ComponentMove(touched)
                                    } else {
                                        self.components.push(Component { pos: pos - vec2(1.0, 1.0), size: vec2(2.0, 2.0) })
                                    }
                                },
                                _ => {}
                            }
                        }
                    }
                }
                ActionState::CableDraw(edited_cables) => {
                    if !response.clicked() {
                        if let Some(pos) = response.hover_pos() {
                            for (id, point_id) in edited_cables.iter_mut() {
                                self.cables.cables[*id]
                                    .move_point_aligned(point_id, (from_screen * pos).round());
                            }
                        }
                    } else {
                        for &(id, _) in edited_cables.iter() {
                            if self.cables.cables.contains_key(id) {
                                self.cables.update_neighbours(id);
                            }
                        }
                        self.action = ActionState::None;
                    }
                },
                ActionState::ComponentMove(id) => {
                    if !response.clicked() {
                        if let Some(pos) = response.hover_pos() {
                            self.components[*id].pos = (from_screen * pos).round() - self.components[*id].size / 2.0
                        }
                    } else {
                        self.action = ActionState::None;
                    }
                }
                _ => {}
            }

            if response.dragged() {
                let delta = response.drag_delta();
                self.bounds
                    .translate(-transform.to_bounds().scale() * delta);
            }

            let scroll = ui.input(|i| i.zoom_delta());
            self.bounds.zoom_in(scroll);

            GridDrawable.show(ui, &transform);
            CablesDrawable::new(&self.cables).show(ui, &transform);
            for component in &self.components {
                ComponentDrawable::new(component).show(ui, &transform);
            }
        });
    }
}
