use super::Component;

#[derive(Debug, Clone, Copy)]
pub struct Constant {
    pub state: bool
}

impl Component for Constant {
    fn propagate(&mut self, _input: &bitvec::slice::BitSlice, output: &mut bitvec::slice::BitSlice) {
        output.set(0, self.state)
    }

    fn input_size() -> usize where Self: Sized {
        0
    }

    fn output_size() -> usize where Self: Sized {
        1
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DebugOutput { pub state: bool }

impl Component for DebugOutput {
    fn propagate(&mut self, input: &bitvec::slice::BitSlice, _output: &mut bitvec::slice::BitSlice) {
        self.state = input[0]
    }

    fn input_size() -> usize where Self: Sized {
        1
    }

    fn output_size() -> usize where Self: Sized {
        0
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    
}