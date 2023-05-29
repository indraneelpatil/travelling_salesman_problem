// Created by Indraneel on 29/05/23
use graph_lib::*;


pub mod prims_mst;
pub mod kruskals_mst;
use prims_mst::*;
use kruskals_mst::*;

fn main() {

    // Create an empty graph
    let mut graph = Graph {
        vertices : Vec::new(),
        edges : Vec::new()
    };

    // Get a random graph
    graph.randomise_graph();

    //println!{"{:?}",graph};

    // Invoke MST Solver
    prims_solver(&mut graph);
    println!("MST cost is {}",graph.calculate_cost());
    // TODO preorder traversal of MST

    // Visualise the graph
    graph.plot_graph().unwrap();
    graph.reset_tsp();
}