pub mod gates;
pub mod simple;

use std::fmt::Debug;

use bitvec::slice::BitSlice;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

use crate::impl_comp_as_ref;

use self::{
    gates::{And, Not, Or, Xor},
    simple::{Constant, DebugOutput, Fork},
};

#[enum_dispatch(Component)]
pub trait ComponentBehaviour: Debug {
    fn propagate(&mut self, input: &BitSlice, output: &mut BitSlice, mask: &mut BitSlice);
    fn input_size(&self) -> usize;
    fn output_size(&self) -> usize;
}

#[enum_dispatch]
#[rustfmt::skip]
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Component {
    And, Or, Xor, Not,
    Fork, DebugOutput, Constant,
}

impl_comp_as_ref![
    And, Or, Xor, Not,
    Fork, DebugOutput, Constant
];