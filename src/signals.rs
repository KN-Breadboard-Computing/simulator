use std::ops::{Index, IndexMut, Range, RangeInclusive};

pub struct ControlSignals {
    bitfield: [u8; 40],
}

macro_rules! gen_shorthands {
    ($(($s:ident,$i:literal,$in:literal)),* ) => {
        impl ControlSignals {
            pub fn init() -> Self {
                let mut bitfield = [0;40];
                $(bitfield[$i] = $in);*;
                Self {bitfield}
            } 
            $(pub const $s : usize = $i;)*
        }
    };
}

gen_shorthands! {
    (LD_A,0,0),       (LD_B,1,0),       (LD_F,2,0),       (ALU_OUT,3,1),    (ALU_0,4,0),          (ALU_1,5,0),          (ALU_2,6,0),      (ALU_3,7,0),      //A
    (PC_PLUS,8,0),    (LD_PC,9,1),      (OUT_PC,10,0),    (STC_PLUS,11,0),  (STC_TICK,12,0),      (LD_STC,13,1),        (OUT_STC,14,1),   (LD_MAR,15,1),    //B
    (LD_MBR,16,0),    (W_IN_OUT,17,0),  (MEM_IN,18,1),    (MEM_OUT,19,1),   (OUT_MBR,20,1),       (MEM_PART,21,0),      (LD_TMP_HI,22,0), (LD_TMP_LO,23,0), //C
    (OUT_TMP_HI,24,1),(OUT_TMP_LO,25,1),(PASS_ADDR,26,1), (PASS_DATA,27,1), (ADDR_OUT_IN,28,0),   (DATA_OUT_IN,29,0),   (LD_IR,30,0),     (RST_MC,31,1),    //D
    (MC_PLUS,32,1),   (OUT_FLAG,33,1),  (USE_SWITCH,34,1)                                                                                                   //E
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

