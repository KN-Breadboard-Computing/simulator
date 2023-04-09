use std::{fmt::Debug, any::Any};

use crate::signals::ControlSignals;

pub mod alu;
pub mod control;
pub mod mem;

type MainBusSize = u8;
type AddressBusSize = u16;
#[derive(Debug)]
pub struct Buses {
    pub main: Bus<MainBusSize>,
    pub address: Bus<AddressBusSize>,
}
pub trait Component : Debug + Any{
    fn react(&mut self, signals: &ControlSignals, bus : &mut Buses);

    ///rip
    fn as_any(&mut self) -> &mut dyn Any;
}

#[derive(Debug)]
pub struct Bus<I> {
    value: Option<I>,
}

impl Buses {
    pub fn init() -> Self {
        Self { main: Bus::init(), address : Bus::init() }
    }
}

impl<I : Copy> Bus<I> {
    pub fn init() -> Self {
        Self { value: None }
    }
    pub fn put(&mut self, value: I) {
        match self.value {
            Some(_) => panic!(),
            None => self.value = Some(value),
        }
    }
    pub fn get(&self) -> I {
        match self.value {
            Some(value) => value,
            None => panic!(),
        }
    }
}
