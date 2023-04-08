use crate::signals::ControlSignals;

pub mod alu;
pub mod control;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pulse {
    Now,
    Clock,
    InvClock,
}

pub struct Buses {
    pub main: Bus
}
pub trait Component {
    fn react(&mut self, signals: &ControlSignals, bus : &mut Buses, pulse: Pulse);
}

pub struct Bus {
    value: Option<usize>,
}

impl Buses {
    pub fn init() -> Self {
        Self { main: Bus::init() }
    }
}

impl Bus {
    pub fn init() -> Self {
        Self { value: None }
    }
    pub fn put(&mut self, value: usize) {
        match self.value {
            Some(_) => panic!(),
            None => self.value = Some(value),
        }
    }
    pub fn get(&self) -> usize {
        match self.value {
            Some(value) => value,
            None => panic!(),
        }
    }
}
