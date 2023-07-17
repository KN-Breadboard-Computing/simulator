use std::io::stdin;

use emulator::{graph::Graph, components::{simple::{Constant, DebugOutput}, gates::And}};

pub fn main() {
    
    let mut graph = Graph::new();

    let a = graph.add_component(Constant { state: false });
    let b = graph.add_component(Constant { state: false });

    let and = graph.add_component(And);

    let out = graph.add_component(DebugOutput { state: false });

    graph.add_connection(a, 0, and, 0);
    graph.add_connection(b, 0, and, 1);

    graph.add_connection(and, 0, out, 0);

    graph.propagate_from(a);
    graph.propagate_from(b);

    println!("A: {}", graph.get_comp(a).state);
    println!("B: {}", graph.get_comp(b).state);
    println!("And");
    println!("Out: {}", graph.get_comp(out).state);

    for line in stdin().lines() {
        let line = line.unwrap();
        let inp = line.trim();
        
        match inp {
            "A" => {
                graph.get_comp_mut(a).state = !graph.get_comp(a).state;
                graph.propagate_from(a);
            },
            "B" => {
                graph.get_comp_mut(b).state = !graph.get_comp(b).state;
                graph.propagate_from(b);
            }
            _ => ()
        }
        
        println!("A: {}", graph.get_comp(a).state);
        println!("B: {}", graph.get_comp(b).state);
        println!("And");
        println!("Out: {}", graph.get_comp(out).state);
    }    
}