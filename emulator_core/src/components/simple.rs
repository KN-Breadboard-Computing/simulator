use bitvec::{slice::BitSlice, bits};
use serde::{Deserialize, Serialize};

use super::ComponentBehaviour;

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Constant {
    #[serde(default)]
    pub state: bool,
}

impl ComponentBehaviour for Constant {
    fn propagate(
        &mut self,
        _prev_input: &BitSlice,
        _input: &BitSlice,
        output: &mut BitSlice,
        _mask: &mut BitSlice,
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

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct DebugOutput {
    #[serde(default)]
    pub state: bool,
}

impl ComponentBehaviour for DebugOutput {
    fn propagate(
        &mut self,
        _prev_input: &BitSlice,
        input: &BitSlice,
        _output: &mut BitSlice,
        _mask: &mut BitSlice,
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

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Fork {
    input_size: u8,
    output_size: u8,
}

impl Fork {
    pub fn new(input_size: u8, output_size: u8) -> Self {
        Self { input_size, output_size }
    }
}

impl ComponentBehaviour for Fork {
    fn propagate(
        &mut self,
        _prev_input: &BitSlice,
        input: &BitSlice,
        output: &mut BitSlice,
        _mask: &mut BitSlice,
    ) {
        let bit = input[..self.input_size as usize].any();
        for i in 0..self.output_size {
            output.set(i as usize, bit);
        }
    }

    fn input_size(&self) -> usize {
        self.input_size as usize
    }

    fn output_size(&self) -> usize {
        self.output_size as usize
    }
}
