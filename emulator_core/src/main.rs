use std::io::stdin;

use emulator_core::{graph::Graph, components::{simple::{Constant, DebugOutput, Fork}, gates::{And, Xor}}};

pub fn main() {
    
    let mut graph = Graph::new();

    let a = graph.add_comp(Constant { state: false });
    let a_fork = graph.add_comp(Fork);
    let b = graph.add_comp(Constant { state: false });
    let b_fork = graph.add_comp(Fork);

    let xor = graph.add_comp(Xor);
    let and = graph.add_comp(And);

    let out_sum = graph.add_comp(DebugOutput { state: false });
    let out_carry = graph.add_comp(DebugOutput { state: false });

    graph.add_conn(a, 0, a_fork, 0);
    graph.add_conn(b, 0, b_fork, 0);

    graph.add_conn(a_fork, 0, and, 0);
    graph.add_conn(a_fork, 1, xor, 0);

    graph.add_conn(b_fork, 0, and, 1);
    graph.add_conn(b_fork, 1, xor, 1);

    graph.add_conn(and, 0, out_carry, 0);
    graph.add_conn(xor, 0, out_sum, 0);

    graph.propagate_from(a);
    graph.propagate_from(b);
    
    for line in stdin().lines() {
        let line = line.unwrap();
        let inp = line.trim();
        
        match inp {
            "A" => {
                graph[a].state = !graph[a].state;
                graph.propagate_from(a);
            },
            "B" => {
                graph[b].state = !graph[b].state;
                graph.propagate_from(b);
            }
            _ => ()
        }
        
        println!("A: {}", graph[a].state);
        println!("B: {}", graph[b].state);
        println!("Adder (A, B -> Forks -> And, Xor )");
        println!("Out: C {} S {}", graph[out_carry].state, graph[out_sum].state);
    }    
}