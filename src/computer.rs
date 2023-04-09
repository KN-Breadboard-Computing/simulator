use std::{
    any::{self, Any},
    iter::once,
};

use crate::{
    components::{alu::Alu, control::ControlUnit, mem::Memory, Buses, Component},
    signals::{ControlSignals, Pulse},
};

pub struct Computer {
    components: Vec<Box<dyn Component>>,
    control: ControlUnit,
    halt: bool,
}

impl Computer {
    pub fn init() -> Self {
        Self {
            components: vec![Box::new(Alu::init()), Box::new(Memory::init(vec![1]))],
            control: ControlUnit::init(),
            halt: false,
        }
    }

    pub fn find_comp<T: 'static>(&mut self) -> &mut T {
        self.components
            .iter_mut()
            .find_map(|bx| bx.as_any().downcast_mut::<T>())
            .unwrap()
    }

    pub fn tick_clock(&mut self) {
        if let Some(signals) = self.control.set_signals() {
            println!("{:064b}", signals.bitfield);

            let mut bus = Buses::init();
            println!("{:?}", self.components);
            println!("{:?}", self.control);
            println!("{:?}", bus);
            
            let filt = signals.filter(ControlSignals::NOW_MASK);
            for comp in self.all_comp() {
                comp.react(&filt, &mut bus);
            }
            // println!("{:?}", self.components);
            // println!("{:?}", self.control);
            // println!("{:?}", bus);
            
            let filt = signals.filter(ControlSignals::CLK_MASK);
            for comp in self.all_comp() {
                comp.react(&filt, &mut bus);
            }
            // println!("{:?}", self.components);
            // println!("{:?}", self.control);
            // println!("{:?}", bus);
            
            let filt = signals.filter(ControlSignals::INVCLK_MASK);
            for comp in self.all_comp() {
                comp.react(&filt, &mut bus);
            }
            println!("{:?}", self.components);
            println!("{:?}", self.control);
            println!("{:?}", bus);
        } else {
            self.halt = true;
        }
    }
    
    pub fn run_clock(&mut self) {
        while !self.halt {
            self.tick_clock();
        }
    }

    fn all_comp(&mut self) -> impl Iterator<Item = &mut (dyn Component + 'static)> {
        self.components
            .iter_mut()
            .map(|bx| bx.as_mut())
            .chain(once(&mut self.control as &mut dyn Component))
    }
}
