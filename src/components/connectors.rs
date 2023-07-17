use super::ComponentBehaviour;

#[derive(Debug, Clone, Copy)]
pub struct Fork;

impl ComponentBehaviour for Fork {
    fn propagate(&mut self, input: &bitvec::slice::BitSlice, output: &mut bitvec::slice::BitSlice) {
        output.fill(input[0]);
    }

    fn input_size() -> usize
    where
        Self: Sized,
    {
        1
    }

    fn output_size() -> usize
    where
        Self: Sized,
    {
        2
    }
}
