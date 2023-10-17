use egui::Ui;

use crate::{
    components::registry::ComponentRegistry,
    state::modes::{AddingOptions, Mode, ModeState},
};

pub fn show_mode_choice(ui: &mut Ui, state: &mut ModeState) {
    ui.radio_value(&mut state.mode, Mode::Running, "Running");
    ui.radio_value(&mut state.mode, Mode::Adding, "Adding");
    ui.radio_value(&mut state.mode, Mode::Editing, "Editing");
    ui.radio_value(&mut state.mode, Mode::Deleting, "Deleting");
}

pub fn show_adding_choice(ui: &mut Ui, state: &mut ModeState, registry: &ComponentRegistry) {
    ui.radio_value(&mut state.add_opt, AddingOptions::Cable, "Cable");
    for (rid, entry) in registry.iter() {
        ui.radio_value(
            &mut state.add_opt,
            AddingOptions::ComponentRid(rid),
            &entry.name,
        );
    }
}
