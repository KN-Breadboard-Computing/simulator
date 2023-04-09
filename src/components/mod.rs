use crate::signals::ControlSignals;

pub mod alu;
pub mod control;

type MainBusSize = u8;
pub struct Buses {
    pub main: Bus<MainBusSize>
}
pub trait Component {
    fn react(&mut self, signals: &ControlSignals, bus : &mut Buses);
}


pub struct Bus<I> {
    value: Option<I>,
}

impl Buses {
    pub fn init() -> Self {
        Self { main: Bus::init() }
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
