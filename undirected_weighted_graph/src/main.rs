use rand::Rng;
use plotters::prelude::*;

// Undirected weighted graph
#[derive(Debug)]
struct Edge {
    vertex_endpoint_1 : u32,
    vertex_endpoint_2 : u32,
    weight : u64
}

#[derive(Debug)]
struct Vertex {
    x : i32,
    y : i32
}

#[derive(Debug)]
struct Graph {
    vertices : Vec<Vertex>,
    edges : Vec<Edge>
}

impl Graph {
    fn plot_graph(&self) -> Result<(), Box<dyn std::error::Error>> {
        let root = BitMapBackend::new("plot.png", (800, 600)).into_drawing_area();
        root.fill(&WHITE)?;
        
        //println!("{}",{(*self.vertices.iter()).map(|vertex| vertex.x)});

        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .margin(5)
            .caption("Plot", ("sans-serif", 50))
            .build_cartesian_2d(1..100, 1..100)?;

        chart.draw_series(
            self.vertices
                .iter()
                .map(|vertex| Circle::new((vertex.x, vertex.y), 5, ShapeStyle::from(&BLACK).filled())),
        )?;

        Ok(())
    }
}

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
