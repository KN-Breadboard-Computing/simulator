use std::iter::once_with;

use crate::components::{Component, control::ControlUnit, Buses, Pulse};

pub struct Computer {
    components: Vec<Box<dyn Component>>,
    control: Box<ControlUnit>,
}

impl Computer {
    pub fn run_clock(&mut self) {
        while let Some(signals) = self.control.set_signals() {
            let mut bus = Buses::init();

            for comp in self.all_comp() {
                comp.react(&signals, &mut bus, Pulse::Now);
            }

            for comp in self.all_comp() {
                comp.react(&signals, &mut bus, Pulse::Clock);
            }

            for comp in self.all_comp() {
                comp.react(&signals, &mut bus, Pulse::InvClock);
            }
        }
    }

    fn all_comp(&mut self) -> impl Iterator<Item = &mut (dyn Component + 'static)> {
        self.components.iter_mut().map(|bx| bx.as_mut()).chain(once_with(|| self.control.as_mut() as &mut dyn Component))
    }
}