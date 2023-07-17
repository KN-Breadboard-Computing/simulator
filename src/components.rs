pub mod gates;
pub mod simple;

use std::{fmt::Debug, any::Any};

use bitvec::slice::BitSlice;

pub trait Component : Debug + Any {
    fn propagate(&mut self, input: &BitSlice, output: &mut BitSlice);
    fn input_size() -> usize where Self: Sized;
    fn output_size() -> usize where Self: Sized;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}