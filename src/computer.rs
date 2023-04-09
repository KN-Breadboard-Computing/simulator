use std::iter::once;

use crate::{components::{Component, control::ControlUnit, Buses}, signals::Pulse};

pub struct Computer {
    components: Vec<Box<dyn Component>>,
    control: Box<ControlUnit>,
}

impl Computer {
    pub fn run_clock(&mut self) {
        while let Some(signals) = self.control.set_signals() {
            let mut bus = Buses::init();

            let filt = signals.filter(Pulse::Now);
            for comp in self.all_comp() {
                comp.react(&filt, &mut bus);
            }
            
            let filt = signals.filter(Pulse::Clock);
            for comp in self.all_comp() {
                comp.react(&filt, &mut bus);
            }
            
            let filt = signals.filter(Pulse::InvClock);
            for comp in self.all_comp() {
                comp.react(&filt, &mut bus);
            }
        }
    }

    fn all_comp(&mut self) -> impl Iterator<Item = &mut (dyn Component + 'static)> {
        self.components.iter_mut().map(|bx| bx.as_mut()).chain(once(self.control.as_mut() as &mut dyn Component))
    }
}