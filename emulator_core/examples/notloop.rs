use emulator_core::{graph::Graph, components::gates::Not};

fn main() {
    let mut graph = Graph::new();

    let not = graph.add_comp(Not);
    graph.add_conn(not, 0, not, 0);

    graph.propagate_from(not);
}