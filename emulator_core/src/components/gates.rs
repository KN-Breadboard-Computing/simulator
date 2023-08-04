use bitvec::slice::BitSlice;
use serde::{Deserialize, Serialize};

use super::ComponentBehaviour;

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct And;

impl ComponentBehaviour for And {
    fn propagate(&mut self, input: &BitSlice, output: &mut BitSlice, mask: &mut BitSlice) {
        output.set(0, input[0] && input[1]);
    }
    fn input_size(&self) -> usize {
        2
    }
    fn output_size(&self) -> usize {
        1
    }
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Or;

impl ComponentBehaviour for Or {
    fn propagate(&mut self, input: &BitSlice, output: &mut BitSlice, mask: &mut BitSlice) {
        output.set(0, input[0] || input[1]);
    }
    fn input_size(&self) -> usize {
        2
    }
    fn output_size(&self) -> usize {
        1
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Not;

impl ComponentBehaviour for Not {
    fn propagate(&mut self, input: &BitSlice, output: &mut BitSlice, mask: &mut BitSlice) {
        output.set(0, !input[0]);
    }
    fn input_size(&self) -> usize {
        1
    }
    fn output_size(&self) -> usize {
        1
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Xor;

impl ComponentBehaviour for Xor {
    fn propagate(&mut self, input: &bitvec::slice::BitSlice, output: &mut bitvec::slice::BitSlice, mask: &mut BitSlice) {
        output.set(0, input[0] ^ input[1]);
    }
    fn input_size(&self) -> usize {
        2
    }
    fn output_size(&self) -> usize {
        1
    }
}
