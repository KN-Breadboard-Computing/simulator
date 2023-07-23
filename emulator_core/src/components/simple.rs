use serde::{Deserialize, Serialize};

use super::ComponentBehaviour;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Constant {
    pub state: bool,
}

impl ComponentBehaviour for Constant {
    fn propagate(
        &mut self,
        _input: &bitvec::slice::BitSlice,
        output: &mut bitvec::slice::BitSlice,
    ) {
        output.set(0, self.state)
    }
    fn input_size(&self) -> usize {
        0
    }
    fn output_size(&self) -> usize {
        1
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DebugOutput {
    pub state: bool,
}

impl ComponentBehaviour for DebugOutput {
    fn propagate(
        &mut self,
        input: &bitvec::slice::BitSlice,
        _output: &mut bitvec::slice::BitSlice,
    ) {
        self.state = input[0]
    }
    fn input_size(&self) -> usize {
        1
    }
    fn output_size(&self) -> usize {
        0
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Fork;

impl ComponentBehaviour for Fork {
    fn propagate(&mut self, input: &bitvec::slice::BitSlice, output: &mut bitvec::slice::BitSlice) {
        output.fill(input[0]);
    }

    fn input_size(&self) -> usize {
        1
    }

    fn output_size(&self) -> usize {
        2
    }
}
