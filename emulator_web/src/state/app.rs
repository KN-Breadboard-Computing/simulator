use crate::{components::registry::ComponentRegistry, nodegraph::graph::NodeGraph};

use super::{modes::ModeState, selection::{Selection, SelectionState}};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AppState {
    pub mode_state: ModeState,
    pub selection_state: SelectionState,
    pub node_graph: NodeGraph,
    #[serde(skip)]
    pub registry: ComponentRegistry,
}