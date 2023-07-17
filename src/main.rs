use std::io::stdin;

use emulator::{graph::Graph, components::{simple::{Constant, DebugOutput}, gates::{And, Xor}, connectors::Fork}};

pub fn main() {
    
    let mut graph = Graph::new();

    let a = graph.add_component(Constant { state: false });
    let a_fork = graph.add_component(Fork);
    let b = graph.add_component(Constant { state: false });
    let b_fork = graph.add_component(Fork);

    let xor = graph.add_component(Xor);
    let and = graph.add_component(And);

    let out_sum = graph.add_component(DebugOutput { state: false });
    let out_carry = graph.add_component(DebugOutput { state: false });

    graph.add_connection(a, 0, a_fork, 0);
    graph.add_connection(b, 0, b_fork, 0);

    graph.add_connection(a_fork, 0, and, 0);
    graph.add_connection(a_fork, 1, xor, 0);

    graph.add_connection(b_fork, 0, and, 1);
    graph.add_connection(b_fork, 1, xor, 1);

    graph.add_connection(and, 0, out_carry, 0);
    graph.add_connection(xor, 0, out_sum, 0);

    graph.propagate_from(a);
    graph.propagate_from(b);
    
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
        println!("Adder (A, B -> Forks -> And, Xor )");
        println!("Out: C {} S {}", graph.get_comp(out_carry).state, graph.get_comp(out_sum).state);
    }    
}