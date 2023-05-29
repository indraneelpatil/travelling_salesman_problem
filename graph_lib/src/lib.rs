// Created by Indraneel on 29/05/23
use plotters::prelude::*;
use rand::Rng;

// Undirected weighted graph
#[derive(Debug)]
pub struct Edge {
    pub endpoint1 : i32,
    pub endpoint2 : i32,
    pub weight : f64,
    pub on_tsp : bool
}

#[derive(Debug)]
pub struct Vertex {
    pub x : f64,
    pub y : f64
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
                .map(|vertex| Circle::new((vertex.x as i32, vertex.y as i32), 5, ShapeStyle::from(&BLACK).filled())),
        )?;

        // Plot edges
        for edge in &self.edges {
            let endpoint1 = &self.vertices[edge.endpoint1 as usize];
            let endpoint2 = &self.vertices[edge.endpoint2 as usize];
            
            let color = if edge.on_tsp { RED } else { BLACK };

            chart.draw_series(LineSeries::new(
                vec![(endpoint1.x as i32, endpoint1.y as i32), (endpoint2.x as i32, endpoint2.y as i32)],
                &color,
            ))?;
        }

        Ok(())
    }

    fn generate_vertices(&mut self, tot_vertices : u32) {

        let mut num_vertices = 0;
        loop {

            // Generate x index
            let rand_x = rand::thread_rng().gen_range(1.0..=100.0);
            // Generate y index
            let rand_y = rand::thread_rng().gen_range(1.0..=100.0);

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

    fn euclidean_distance(&self,x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
        let x_diff = x2 - x1;
        let y_diff = y2 - y1;
        let distance_squared = x_diff * x_diff + y_diff * y_diff;
        distance_squared.sqrt()
    }

    fn generate_edges(&mut self, num_edges: u32, num_vertices: u32, max_weight: f64) {
        
        let mut generated_weights = Vec::new();
        
        while self.edges.len() < num_edges as usize {
            let endpoint1 = rand::thread_rng().gen_range(0..=num_vertices) as i32;
            let endpoint2 = rand::thread_rng().gen_range(0..=num_vertices) as i32;
            // weight is manhattan distance between the two vertices
            let pt1 = &self.vertices[endpoint1 as usize];
            let pt2 = &self.vertices[endpoint2 as usize];
            let weight = self.euclidean_distance(pt1.x, pt1.y, pt2.x, pt2.y);
            //let weight = rand::thread_rng().gen_range(0..=max_weight);
            
            let edge = Edge {
                endpoint1,
                endpoint2,
                weight,
                on_tsp : false
            };
            
            if !self.is_duplicate_edge(&edge) && !generated_weights.contains(&weight) && edge.weight < max_weight{
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
        let tot_edges  = 100;
        let tot_vertices = 50; 
        let max_weight = 30.0;

        // Add random vertices to the graph
        self.generate_vertices(tot_vertices);

        // Add random edges to the graph 
        self.generate_edges(tot_edges,tot_vertices,max_weight);


    }

    pub fn reset_tsp(&mut self) {
        for edge in &mut self.edges {
            edge.on_tsp = false;
        }
    }
    
    pub fn calculate_cost(&self) -> f64 {
        let mut cost = 0.0;
        for edge in &self.edges {
            if edge.on_tsp {
                cost += edge.weight;
            }
        }
        cost
    }
}