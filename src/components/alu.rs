use super::*;

pub struct Alu {
    a: usize,
    b: usize,
}

impl Component for Alu {
    fn react(&mut self, signals: &ControlSignals, bus : &mut Buses, pulse: Pulse) {
        match pulse {
            Pulse::Now => {
                if signals[ControlSignals::ALU_OUT] == 0 {
                    match signals[ControlSignals::ALU_0..=ControlSignals::ALU_3] {
                        [0,0,0,0] => bus.main.put(self.a),
                        [0,0,0,1] => bus.main.put(self.b),
                        [0,0,1,0] => bus.main.put(self.a + self.b),
                        _ => (),
                    }
                }
            },
            Pulse::Clock => {
                if signals[ControlSignals::LD_A] == 1 {
                    self.a = bus.main.get();
                }
                if signals[ControlSignals::LD_B] == 1 {
                    self.b = bus.main.get();
                }
            }
            _ => ()
        }
    }
}