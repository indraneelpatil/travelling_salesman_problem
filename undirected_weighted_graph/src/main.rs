// Created by Indraneel on 29/05/23
use graph_lib::*;

fn main() {

    // Create an empty graph
    let mut graph = Graph {
        vertices : Vec::new(),
        edges : Vec::new()
    };

    graph.randomise_graph();

    println!{"{:?}",graph};

    // Visualise the graph
    graph.plot_graph().unwrap();
}
