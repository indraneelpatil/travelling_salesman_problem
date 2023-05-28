use rand::Rng;

pub mod graph;

use graph::*;

fn main() {
    println!("Hello, world!");

    // Create an empty graph
    let mut graph = Graph {
        vertices : Vec::new(),
        edges : Vec::new()
    };

    // Add random vertices to the graph 
    let mut num_vertices = 0;
    loop {

        // Generate x index
        let rand_x = rand::thread_rng().gen_range(1..=100);
        // Generate y index
        let rand_y = rand::thread_rng().gen_range(1..=100);

        let rand_vert = Vertex {
            x : rand_x,
            y : rand_y
        };

        // Add to graph
        graph.vertices.push(rand_vert);

        if num_vertices == 50 { 
            break;
        }
        num_vertices += 1;
    }

    println!{"{:?}",graph};

    // Visualise the graph
    graph.plot_graph().unwrap();
}
