use super::Component;

#[derive(Debug)]
pub struct Memory {
    data: Vec<u8>,
    mar: u16,
}

impl Memory {
    pub fn init(data : Vec<u8>) -> Self {
        Self { data, mar: 0 }
    }
}

impl Component for Memory {
    fn react(&mut self, signals: &crate::signals::ControlSignals, bus : &mut super::Buses) {
        use crate::signals::con::*;
        
        if signals.get(LD_MAR) == 1 {
            self.mar = bus.address.get();
        }
        if signals.get(MEM_OUT) == 0 {
            bus.main.put(self.data[self.mar as usize]);
        }

        //TODO: implement other signals
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
}