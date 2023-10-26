use crate::components::registry::ComponentRid;

#[derive(Debug, Default, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct ModeState {
    pub mode : Mode,
    pub add_opt: AddingOptions,
    pub edit_opt: EditingOptions,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub enum Mode {
    #[default]
    Running,
    Adding,
    Editing,
    Deleting
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AddingOptions {
    #[default]
    Cable,
    ComponentRid(ComponentRid)
}

/// TODO
/// Box editing
#[derive(Debug, Clone, Copy, Default, serde::Serialize, serde::Deserialize)]
pub enum EditingOptions {
    #[default]
    Basic
}