use super::ComponentBehaviour;

#[derive(Debug, Clone, Copy)]
pub struct And;

impl ComponentBehaviour for And {
    fn propagate(&mut self, input: &bitvec::slice::BitSlice, output: &mut bitvec::slice::BitSlice) {
        output.set(0, input[0] && input[1]);
    }
    fn input_size() -> usize where Self: Sized { 2 }
    fn output_size() -> usize where Self: Sized { 1 }
}

#[derive(Debug, Clone, Copy)]
pub struct Or;

impl ComponentBehaviour for Or {
    fn propagate(&mut self, input: &bitvec::slice::BitSlice, output: &mut bitvec::slice::BitSlice) {
        output.set(0, input[0] || input[1]);
    }
    fn input_size() -> usize where Self: Sized { 2 }
    fn output_size() -> usize where Self: Sized { 1 }
}

#[derive(Debug, Clone, Copy)]
pub struct Not;

impl ComponentBehaviour for Not {
    fn propagate(&mut self, input: &bitvec::slice::BitSlice, output: &mut bitvec::slice::BitSlice) {
        output.set(0, !input[0]);
    }
    fn input_size() -> usize where Self: Sized { 1 }
    fn output_size() -> usize where Self: Sized { 1 }
}

#[derive(Debug, Clone, Copy)]
pub struct Xor;

impl ComponentBehaviour for Xor {
    fn propagate(&mut self, input: &bitvec::slice::BitSlice, output: &mut bitvec::slice::BitSlice) {
        output.set(0, input[0] ^ input[1]);
    }
    fn input_size() -> usize where Self: Sized { 2 }
    fn output_size() -> usize where Self: Sized { 1 }
}

