use super::Component;

#[derive(Debug, Clone, Copy)]
pub struct And;

impl Component for And {
    fn propagate(&mut self, input: &bitvec::slice::BitSlice, output: &mut bitvec::slice::BitSlice) {
        output.set(0, input[0] && input[1]);
    }
    fn input_size() -> usize where Self: Sized { 2 }
    fn output_size() -> usize where Self: Sized { 1 }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}