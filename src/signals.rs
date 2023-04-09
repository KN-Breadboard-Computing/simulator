use std::ops::{Index, IndexMut, Range, RangeInclusive};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pulse {
    Now,
    Clock,
    InvClock,
}

#[derive(Debug, Clone)]
pub struct ControlSignals {
    bitfield: u64,
}

macro_rules! gen_shorthands {
    ($(($s:ident,$i:literal,$in:literal,$cl:ident)),* ) => {
        impl ControlSignals {
            pub fn init() -> Self {
                let mut sig = Self {bitfield : 0};
                $(sig.set($i,$in));*;
                sig
            }

            const fn pulse_mask(clk: Pulse) -> u64 {
                let mut filt = 0;
                $(
                    if (clk as usize) == ($cl as usize) {
                        filt = bit_set(filt,$i,1);
                    }
                );*
                filt
            }
        }
        pub mod con {
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

impl ControlSignals {
    pub const NOW_MASK: u64 = Self::pulse_mask(Pulse::Now);
    pub const CLK_MASK: u64 = Self::pulse_mask(Pulse::Clock);
    pub const INVCLK_MASK: u64 = Self::pulse_mask(Pulse::InvClock);

    pub fn from_bits(bits: u64) -> Self {
        Self { bitfield: bits }
    }

    pub fn get(&self, index: usize) -> u64 {
        assert!(index < 64);
        (self.bitfield >> (63 - index)) & 1
    }

    pub fn get_range_inc(&self, index: RangeInclusive<usize>) -> u64 {
        assert!(index.end() < &64);
        // println!("{index:?}");
        // println!("{:064b}", self.bitfield);
        // println!("{:064b}", self.bitfield << index.start());
        // println!("{:064b}", (self.bitfield << index.start()) >> (63 - index.end()));
        (self.bitfield << index.start()) >> (63 - (index.end() - index.start()))
    }

    pub fn set(&mut self, index: usize, bit: u64) {
        assert!(bit == 0 || bit == 1);
        let shifted = bit << (63 - index);
        self.bitfield &= !shifted;
        self.bitfield |= shifted;
    }

    pub fn filter(&self, mask: u64) -> Self {
        Self {
            bitfield: self.bitfield & mask,
        }
    }
}

const fn bit_set(bits: u64, index: usize, bit: u64) -> u64 {
    assert!(bit == 0 || bit == 1);
    let shifted = bit << (63 - index);
    (bits & !shifted) | shifted
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_test() {
        let sig = ControlSignals::init();

        for i in 0..64 {
            println!("{}", sig.get(i));
        }
        panic!();
    }

    #[test]
    fn get_range_test() {
        let sig = ControlSignals::init();

        println!("Ref: {:064b}", sig.bitfield);

        for i in 0..60 {
            println!("{:064b}", sig.get_range_inc(i..=(i + 4)));
        }

        panic!();
    }

    #[test]
    fn mask_test() {
        println!("{:064b}", ControlSignals::NOW_MASK);
        println!("{:064b}", ControlSignals::CLK_MASK);
        println!("{:064b}", ControlSignals::INVCLK_MASK);
        panic!();
    }
}

// impl Index<usize> for ControlSignals {
//     type Output = u64;

//     fn index(&self, index: usize) -> &Self::Output {
//         &((self.bitfield >> index) & 1)
//     }
// }

// impl Index<Range<usize>> for ControlSignals {
//     type Output = u64;

//     fn index(&self, index: Range<usize>) -> &Self::Output {
//         &((self.bitfield >> index.start) & (!(!0 << index.end)))
//     }
// }

// impl Index<RangeInclusive<usize>> for ControlSignals {
//     type Output = [u8];

//     fn index(&self, index: RangeInclusive<usize>) -> &Self::Output {
//         &self.bitfield[index]
//     }
// }

// impl IndexMut<usize> for ControlSignals {
//     fn index_mut(&mut self, index: usize) -> &mut Self::Output {
//         self.bitfield
//     }
// }

// impl IndexMut<Range<usize>> for ControlSignals {
//     fn index_mut(&mut self, index: Range<usize>) -> &mut Self::Output {
//         &mut self.bitfield[index]
//     }
// }

// impl IndexMut<RangeInclusive<usize>> for ControlSignals {
//     fn index_mut(&mut self, index: RangeInclusive<usize>) -> &mut Self::Output {
//         &mut self.bitfield[index]
//     }
// }
