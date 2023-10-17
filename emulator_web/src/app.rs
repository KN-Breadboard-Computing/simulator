use egui::{Sense, warn_if_debug_build};
use log::info;

use crate::{
    state::{self, modes::Mode, app::AppState},
    widgets::{ui::side_menu, nodegraph::{self, widget::nodegraph_widget}},
};

#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct EmulatorApp {
    // pub bounds: NodeGraphBounds,
    // pub graph: NodeGraph,
    // pub mode: Mode,
    // pub choosen_component: ChoosenComponent,
    // #[serde(skip)]
    // pub action: ActionState,
    app_state: state::app::AppState,
}

impl EmulatorApp {
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

impl eframe::App for EmulatorApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::new(egui::panel::Side::Left, "Side Panel").resizable(true).show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                if ui.button("Clear").clicked() {
                    self.app_state = AppState::default();
                }
            });
            ui.separator();
            side_menu::show_mode_choice(ui, &mut self.app_state.mode_state);
            if self.app_state.mode_state.mode == Mode::Adding {
                ui.separator();
                side_menu::show_adding_choice(
                    ui,
                    &mut self.app_state.mode_state,
                    &self.app_state.registry,
                );
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            warn_if_debug_build(ui);
            nodegraph_widget(ui, &mut self.app_state);
            // let size = ui.available_size();
            // let response = ui.allocate_response(size, Sense::click_and_drag());

            // handle_actions(self, &response);

            // let transform = NodeGraphTransform::new(response.rect, self.bounds.clone());

            // if response.dragged() {
            //     let delta = response.drag_delta();
            //     self.bounds
            //         .translate(-transform.to_bounds().scale() * delta);
            // }

            // let scroll = ui.input(|i| i.zoom_delta());
            // self.bounds.zoom_in(scroll);

            // GridDrawable.show(ui, &transform);
            // CablesDrawable::new(&mut self.graph.cables).show(ui, &transform);
            // for (id, component) in &self.graph.components {
            //     ComponentDrawable::new(component, &self.graph.logical_graph[id])
            //         .show(ui, &transform);
            // }
        });
    }
}
