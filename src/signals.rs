use std::ops::{Index, IndexMut, Range, RangeInclusive};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pulse {
    Now,
    Clock,
    InvClock,
}

#[derive(Debug, Clone)]
pub struct ControlSignals {
    bitfield: [u8; 40],
}

macro_rules! gen_shorthands {
    ($(($s:ident,$i:literal,$in:literal,$cl:ident)),* ) => {
        impl ControlSignals {
            pub fn init() -> Self {
                let mut bitfield = [0;40];
                $(bitfield[$i] = $in);*;
                Self {bitfield}
            } 
            pub fn filter(&self, clk: Pulse) -> Self {
                let mut cp = self.to_owned();
                $(
                    if clk != $cl {
                        cp[$i] = 0;
                    }
                );*
                cp
            }
            $(pub const $s : usize = $i;)*
        }
    };
}
 
use Pulse::*;
gen_shorthands! {
    (LD_A,0,0,Clock),       (LD_B,1,0,Clock),       (LD_F,2,0,Clock),       (ALU_OUT,3,1,Now),    (ALU_0,4,0,Now),          (ALU_1,5,0,Now),          (ALU_2,6,0,Now),      (ALU_3,7,0,Now),      //A
    (PC_PLUS,8,0,InvClock),    (LD_PC,9,1,Now),      (OUT_PC,10,0,Now),    (STC_PLUS,11,0,Now),  (STC_TICK,12,0,InvClock),      (LD_STC,13,1,Now),        (OUT_STC,14,1,Now),   (LD_MAR,15,1,Clock),    //B
    (LD_MBR,16,0,Clock),    (W_IN_OUT,17,0,Now),  (MEM_IN,18,1,Now),    (MEM_OUT,19,1,Now),   (OUT_MBR,20,1,Now),       (MEM_PART,21,0,Now),      (LD_TMP_HI,22,0,Clock), (LD_TMP_LO,23,0,Clock), //C
    (OUT_TMP_HI,24,1,Now),(OUT_TMP_LO,25,1,Now),(PASS_ADDR,26,1,Now), (PASS_DATA,27,1,Now), (ADDR_OUT_IN,28,0,Now),   (DATA_OUT_IN,29,0,Now),   (LD_IR,30,0,InvClock),     (RST_MC,31,1,Now),    //D
    (MC_PLUS,32,1,InvClock),   (OUT_FLAG,33,1,Now),  (USE_SWITCH,34,1,Now)                                                                                                   //E
}


impl Index<usize> for ControlSignals {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.bitfield[index]
    }
}

impl Index<Range<usize>> for ControlSignals {
    type Output = [u8];

    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.bitfield[index]
    }
}

impl Index<RangeInclusive<usize>> for ControlSignals {
    type Output = [u8];

    fn index(&self, index: RangeInclusive<usize>) -> &Self::Output {
        &self.bitfield[index]
    }
}

impl IndexMut<usize> for ControlSignals {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.bitfield[index]
    }
}

impl IndexMut<Range<usize>> for ControlSignals {
    fn index_mut(&mut self, index: Range<usize>) -> &mut Self::Output {
        &mut self.bitfield[index]
    }
}

impl IndexMut<RangeInclusive<usize>> for ControlSignals {
    fn index_mut(&mut self, index: RangeInclusive<usize>) -> &mut Self::Output {
        &mut self.bitfield[index]
    }
}

