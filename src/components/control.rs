use super::*;

pub struct ControlUnit {
    micro_count: usize,
    i_reg: usize,
}

impl ControlUnit {
    pub fn set_signals(&mut self) -> Option<ControlSignals> {
        //Table of microcodes  
        //(microcode,i_reg) -> signals 
        todo!()
    }
}

impl Component for ControlUnit {
    fn react(&mut self, signals: &ControlSignals, bus : &mut Buses) {
        todo!()
    }
}