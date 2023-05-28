use graph_lib::*;

fn main() {
    println!("Hello, world!");

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
