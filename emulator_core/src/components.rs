pub mod gates;
pub mod simple;
pub mod connectors;

use std::{any::Any, fmt::Debug};

use bitvec::slice::BitSlice;

pub trait ComponentBehaviour: Debug {
    fn propagate(&mut self, input: &BitSlice, output: &mut BitSlice);
    fn input_size() -> usize
    where
        Self: Sized;
    fn output_size() -> usize
    where
        Self: Sized;
}

pub trait Component: Debug {
    fn propagate(&mut self, input: &BitSlice, output: &mut BitSlice);
    fn input_size() -> usize
    where
        Self: Sized;
    fn output_size() -> usize
    where
        Self: Sized;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: ComponentBehaviour + Any> Component for T {
    fn propagate(&mut self, input: &BitSlice, output: &mut BitSlice) {
        self.propagate(input, output)
    }

    fn input_size() -> usize
    where
        Self: Sized,
    {
        Self::input_size()
    }

    fn output_size() -> usize
    where
        Self: Sized,
    {
        Self::output_size()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
