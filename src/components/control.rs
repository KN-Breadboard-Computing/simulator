use std::{u8, collections::HashMap};

use super::*;


#[derive(Debug, Hash, PartialEq, Eq)]
pub struct ControlUnit {
    micro_count: u8,
    i_reg: u8,
    program_count: u16,
}

impl ControlUnit {
    pub fn init() -> Self {
        Self { micro_count: 0, i_reg: 0, program_count: 0 }
    }

    pub fn set_signals(&mut self) -> Option<ControlSignals> {
        //Table of microcodes  
        //(i_reg,micro_count) -> signals 
        
        let map = build_map();

        let bits = map.get(&(self.i_reg, self.micro_count));

        Some(ControlSignals::from_bits(*bits.unwrap()))
    }
}

impl Component for ControlUnit {
    fn react(&mut self, signals: &ControlSignals, bus : &mut Buses) {
        use crate::signals::con::*;

        if signals.get(PC_PLUS) == 1 {
            self.program_count += 1;
        }
        if signals.get(OUT_PC) == 0 {
            bus.address.put(self.program_count);
        }
        if signals.get(LD_PC) == 0 {
            self.program_count = bus.address.get();
        }
        if signals.get(MC_PLUS) == 1 {
            self.micro_count += 1;
        }
        if signals.get(RST_MC) == 0 {
            self.micro_count = 0;
        }
        if signals.get(LD_IR) == 1 {
            self.i_reg = bus.main.get();
        }
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}


#[allow(clippy::unusual_byte_groupings)]
fn build_map() -> HashMap<(u8,u8), u64> {
    let mut map = HashMap::new();

    //A
    //{ 0b0000000010000, 0b00010000 },  // MOVAB - MICROSTEP 0
    //{ 0b0000000010001, 0b00010000 },  // MOVAB - MICROSTEP 1
    //{ 0b0000000010010, 0b10000010 },  // MOVAB - MICROSTEP 2
    //{ 0b0000000010011, 0b00010000 },  // MOVAB - MICROSTEP 3

    //B
    // { 0b0000000010000, 0b01000111 },  // MOVAB - MICROSTEP 0
    // { 0b0000000010001, 0b11100110 },  // MOVAB - MICROSTEP 1
    // { 0b0000000010010, 0b01100110 },  // MOVAB - MICROSTEP 2
    // { 0b0000000010011, 0b01100110 },  // MOVAB - MICROSTEP 3

    //C
    // { 0b0000000010000, 0b00111000 },  // MOVAB - MICROSTEP 0
    // { 0b0000000010001, 0b00101000 },  // MOVAB - MICROSTEP 1
    // { 0b0000000010010, 0b00111000 },  // MOVAB - MICROSTEP 2
    // { 0b0000000010011, 0b00111000 },  // MOVAB - MICROSTEP 3

    //D
    // { 0b0000000010000, 0b11110001 },  // MOVAB - MICROSTEP 0
    // { 0b0000000010001, 0b11110011 },  // MOVAB - MICROSTEP 1
    // { 0b0000000010010, 0b11110001 },  // MOVAB - MICROSTEP 2
    // { 0b0000000010011, 0b11110000 },  // MOVAB - MICROSTEP 3

    //E
    // { 0b0000000010000, 0b11000000 },  // MOVAB - MICROSTEP 0
    // { 0b0000000010001, 0b11000000 },  // MOVAB - MICROSTEP 1
    // { 0b0000000010010, 0b11000000 },  // MOVAB - MICROSTEP 2
    // { 0b0000000010011, 0b11000000 },  // MOVAB - MICROSTEP 3

    map.insert((0b0,0b00), 0b00010000_01000111_00111000_11110001_11000000 << (64 - 40));
    map.insert((0b0,0b01), 0b00010000_11100110_00101000_11110011_11000000 << (64 - 40));
    
    map.insert((0b1,0b00), 0b00010000_01000111_00111000_11110001_11000000 << (64 - 40));
    map.insert((0b1,0b01), 0b00010000_11100110_00101000_11110011_11000000 << (64 - 40));
    
    map.insert((0b1,0b01), 0b10000010_01100110_00111000_11110001_11000000 << (64 - 40));
    map.insert((0b1,0b10), 0b00010000_01100110_00111000_11110000_11000000 << (64 - 40));

    //TODO: A way to load eeprom data

    map
}