use super::*;

type AluSize = u8;

pub struct Alu {
    a: AluSize,
    b: AluSize,
}

impl Component for Alu {
    fn react(&mut self, signals: &ControlSignals, bus: &mut Buses) {
        if signals[ControlSignals::ALU_OUT] == 0 {
            let alu_res = match signals[ControlSignals::ALU_0..=ControlSignals::ALU_3] {
                [0, 0, 0, 0] => 0,
                [0, 0, 0, 1] => self.a,
                [0, 0, 1, 0] => self.b,
                [0, 0, 1, 1] => self.a.wrapping_add(self.b),
                
                [0, 1, 0, 0] => self.a.wrapping_neg(),
                [0, 1, 0, 1] => self.b.wrapping_neg(),
                [0, 1, 1, 0] => self.a.wrapping_sub(self.b),
                [0, 1, 1, 1] => self.b.wrapping_sub(self.a),
                
                [1, 0, 0, 0] => !self.a,
                [1, 0, 0, 1] => !self.b,
                [1, 0, 1, 0] => self.a | self.b,
                [1, 0, 1, 1] => self.a & self.b,

                [1, 1, 0, 0] => self.a << 1,
                [1, 1, 0, 1] => self.b << 1,
                [1, 1, 1, 0] => self.a >> 1,
                [1, 1, 1, 1] => self.b >> 1,

                _ => unreachable!()
            };

            //TODO: flags

            bus.main.put(alu_res);
        }
        if signals[ControlSignals::LD_A] == 1 {
            self.a = bus.main.get();
        }
        if signals[ControlSignals::LD_B] == 1 {
            self.b = bus.main.get();
        }
    }
}


#[cfg(test)]
mod tests {
    
}