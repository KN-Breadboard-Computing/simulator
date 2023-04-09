use emulator::{computer::Computer, components::alu::Alu};

fn main() {
    let mut comp = Computer::init();

    let alu = comp.find_comp::<Alu>();
    
    *alu.debug_regs().1 = 10;
    println!("{alu:?}");
    
    comp.tick_clock();
    comp.tick_clock();
    comp.tick_clock();
    comp.tick_clock();
    comp.tick_clock();
    comp.tick_clock();
    comp.tick_clock();
    comp.tick_clock();
    comp.tick_clock();
    comp.tick_clock();
    comp.tick_clock();
    comp.tick_clock();
    
    let alu = comp.find_comp::<Alu>();
    println!("{alu:?}");
}
