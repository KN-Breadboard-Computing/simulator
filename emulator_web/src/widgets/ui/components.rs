use egui::Ui;

use crate::{components::registry::{ComponentRegistry, ComponentRid, ComponentEntry}, nodegraph::components::ComponentNode};

pub fn show_default_component_menu(
    ui: &mut Ui,
    node: &mut ComponentNode,
    comp: &mut emulator_core::components::Component,
    entry: &ComponentEntry
) {
    ui.label(format!("{:?}", node));
    ui.label(format!("{:?}", comp));
    ui.label(format!("{:?}", entry));
}
