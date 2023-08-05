use std::io::stdin;

use emulator_core::{graph::Graph, components::{simple::{Constant, DebugOutput, Fork}, gates::{Or, Not}}};

pub fn main() {
    
    let mut graph = Graph::new();

    let r = graph.add_comp(Constant { state: false });
    let s = graph.add_comp(Constant { state: false });

    let top_or = graph.add_comp(Or);
    let top_not = graph.add_comp(Not);
    let top_fork = graph.add_comp(Fork);

    graph.add_conn(r, 0, top_or, 0);
    graph.add_conn(top_or, 0, top_not, 0);
    graph.add_conn(top_not, 0, top_fork, 0);

    let bot_or = graph.add_comp(Or);
    let bot_not = graph.add_comp(Not);
    let bot_fork = graph.add_comp(Fork);

    graph.add_conn(s, 0, bot_or, 0);
    graph.add_conn(bot_or, 0, bot_not, 0);
    graph.add_conn(bot_not, 0, bot_fork, 0);

    let out_q = graph.add_comp(DebugOutput { state: false });
    let out_not_q = graph.add_comp(DebugOutput { state: false });

    graph.add_conn(top_fork, 0, out_q, 0);
    graph.add_conn(top_fork, 1, bot_or, 1);
    graph.add_conn(bot_fork, 0, out_not_q, 0);
    graph.add_conn(bot_fork, 1, top_or, 1);

    graph.propagate_from(r);
    graph.propagate_from(top_or);
    graph.propagate_from(top_not);
    graph.propagate_from(top_fork);
    graph.propagate_from(s);
    graph.propagate_from(bot_or);
    graph.propagate_from(bot_not);
    graph.propagate_from(bot_fork);
    
    for line in stdin().lines() {
        let line = line.unwrap();
        let inp = line.trim();
        
        match inp {
            "R" => {
                graph[r].state = !graph[r].state;
                graph.propagate_from(r);
            },
            "S" => {
                graph[s].state = !graph[s].state;
                graph.propagate_from(s);
            }
            "D" => {
                dbg!(&graph);
            }
            _ => ()
        }
        
        println!("R: {}", graph[r].state);
        println!("S: {}", graph[s].state);
        println!("Top Nor inputs: {} {}", graph.inputs[top_or.into()][0], graph.inputs[top_or.into()][1]);
        println!("Top Nor output: {}", graph.outputs[top_not.into()][0]);
        println!("Bot Nor inputs: {} {}", graph.inputs[bot_or.into()][0], graph.inputs[bot_or.into()][1]);
        println!("Bot Nor output: {}", graph.outputs[bot_not.into()][0]);
        println!("SR Latch (S, R -> Nor↺ -> Q, Not_Q )");
        println!("Out: Q {} Not_Q {}", graph[out_q].state, graph[out_not_q].state);
    }    
}