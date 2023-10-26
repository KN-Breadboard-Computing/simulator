use std::fmt::Debug;

use simulator_core::components::{
    gates::{And, Not, Or, Xor},
    simple::{Constant, DebugOutput},
    Component,
};

pub struct ComponentEntry {
    pub name: String,
    pub create: Box<dyn Fn() -> simulator_core::components::Component>,
}

/// TODO
/// Custom user components
/// Serialized wasm ( or javascript code? )
/// Solution 1:
/// Core Component has enum variant "Custom" with an id to the Registry (so the graph is easily serializable)
/// Registry is then supplied by the frontend and contains propagation function and other info
/// So registry serializing and deserializing to functions is handled by the frontend
#[derive(Debug)]
pub struct ComponentRegistry(Vec<ComponentEntry>);

pub type ComponentRid = usize;

fn new_entry<F, C>(name: &str, create: F) -> ComponentEntry
where
    F: Fn() -> C + 'static,
    C: Into<Component>,
{
    ComponentEntry {
        name: name.to_string(),
        create: Box::new(move || create().into()),
    }
}

impl ComponentRegistry {
    pub fn base() -> Self {
        Self(vec![
            new_entry("And", And::default),
            new_entry("Or", Or::default),
            new_entry("Not", Not::default),
            new_entry("Xor", Xor::default),
            new_entry("Constant", Constant::default),
            new_entry("Debug Output", DebugOutput::default),
        ])
    }

    pub fn entry(&self, rid: ComponentRid) -> Option<&ComponentEntry> {
        self.0.get(rid)
    }

    pub fn iter(&self) -> impl Iterator<Item = (ComponentRid, &ComponentEntry)> + '_ {
        self.0.iter().enumerate()
    }
}

impl Default for ComponentRegistry {
    fn default() -> Self {
        Self::base()
    }
}

impl Debug for ComponentEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ComponentEntry")
            .field("name", &self.name)
            .field("create", &(self.create)())
            .finish()
    }
}
