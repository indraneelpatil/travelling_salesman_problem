use plotters::prelude::*;
use rand::Rng;

// Undirected weighted graph
#[derive(Debug)]
pub struct Edge {
    endpoint1 : u32,
    endpoint2 : u32,
    weight : u32
}

#[derive(Debug)]
pub struct Vertex {
    pub x : i32,
    pub y : i32
}

#[derive(Debug)]
pub struct Graph {
    pub vertices : Vec<Vertex>,
    pub edges : Vec<Edge>
}

impl Graph {
    pub fn plot_graph(&self) -> Result<(), Box<dyn std::error::Error>> {
        let root = BitMapBackend::new("plot.png", (800, 600)).into_drawing_area();
        root.fill(&WHITE)?;
        
        //println!("{}",{(*self.vertices.iter()).map(|vertex| vertex.x)});

        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .margin(5)
            .caption("TSP", ("sans-serif", 50))
            .build_cartesian_2d(1..100, 1..100)?;

        chart.draw_series(
            self.vertices
                .iter()
                .map(|vertex| Circle::new((vertex.x, vertex.y), 5, ShapeStyle::from(&BLACK).filled())),
        )?;

        // Plot edges
        for edge in &self.edges {
            let endpoint1 = &self.vertices[edge.endpoint1 as usize];
            let endpoint2 = &self.vertices[edge.endpoint2 as usize];

            chart.draw_series(LineSeries::new(
                vec![(endpoint1.x, endpoint1.y), (endpoint2.x, endpoint2.y)],
                &RED,
            ))?;
        }

        Ok(())
    }

    fn generate_vertices(&mut self, tot_vertices : u32) {

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
            self.vertices.push(rand_vert);

            if num_vertices == tot_vertices { 
                break;
            }
            num_vertices += 1;
        }
    }

    fn generate_edges(&mut self, num_edges: u32, num_vertices: u32, max_weight: u32) {
        
        let mut generated_weights = Vec::new();
        
        while self.edges.len() < num_edges as usize {
            let endpoint1 = rand::thread_rng().gen_range(0..=num_vertices);
            let endpoint2 = rand::thread_rng().gen_range(0..=num_vertices);
            // weight is manhattan distance between the two vertices
            let pt1 = &self.vertices[endpoint1 as usize];
            let pt2 = &self.vertices[endpoint2 as usize];
            let weight = rand::thread_rng().gen_range(0..=max_weight);
            
            let edge = Edge {
                endpoint1,
                endpoint2,
                weight,
            };
            
            if !self.is_duplicate_edge(&edge) && !generated_weights.contains(&weight) {
                self.edges.push(edge);
                generated_weights.push(weight);
            }
        }
    }

    fn is_duplicate_edge(&self, edge: &Edge) -> bool {
        self.edges.iter().any(|existing_edge| {
            (existing_edge.endpoint1 == edge.endpoint1 && existing_edge.endpoint2 == edge.endpoint2) ||
            (existing_edge.endpoint1 == edge.endpoint2 && existing_edge.endpoint2 == edge.endpoint1)
        })
    }


    pub fn randomise_graph(&mut self) {

        // parameters
        let tot_edges  = 50;
        let tot_vertices = 50; 

        // Add random vertices to the graph
        self.generate_vertices(tot_vertices);

        // Add random edges to the graph 
        self.generate_edges(tot_edges,tot_vertices,1000);


    }
}