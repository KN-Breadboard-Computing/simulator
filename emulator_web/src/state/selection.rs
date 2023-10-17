use emulator_core::graph::id::ComponentId;

use crate::nodegraph::cables::CableId;

#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct SelectionState {
   pub action: SelectionAction,
   pub selection: Selection,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum SelectionAction {
    #[default]
    Nothing,
    Moving,
}

#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Selection {
    pub cables: Vec<(CableId, usize)>,
    pub comps: Vec<ComponentId>
}

impl Selection {
    pub fn is_empty(&self) -> bool {
        self.cables.is_empty() && self.comps.is_empty()
    }

}