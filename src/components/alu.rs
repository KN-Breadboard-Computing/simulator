use crate::signals::con::*;
use super::*;

type AluSize = u8;

pub struct Alu {
    a: AluSize,
    b: AluSize,
}

impl Component for Alu {
    fn react(&mut self, signals: &ControlSignals, bus: &mut Buses) {
        if signals.get(ALU_OUT) == 0 {
            let alu_res = match signals.get_range_inc(ALU_0..=ALU_3) {
                0 => 0,
                1 => self.a,
                2 => self.b,
                3 => self.a.wrapping_add(self.b),
                
                4 => self.a.wrapping_neg(),
                5 => self.b.wrapping_neg(),
                6 => self.a.wrapping_sub(self.b),
                7 => self.b.wrapping_sub(self.a),
                
                8 => !self.a,
                9 => !self.b,
                10 => self.a | self.b,
                11 => self.a & self.b,

                12 => self.a << 1,
                13 => self.b << 1,
                14 => self.a >> 1,
                15 => self.b >> 1,

                _ => unreachable!()
            };

            //TODO: flags

            bus.main.put(alu_res);
        }
        if signals.get(LD_A) == 1 {
            self.a = bus.main.get();
        }
        if signals.get(LD_B) == 1 {
            self.b = bus.main.get();
        }
    }
}


#[cfg(test)]
mod tests {
    
}