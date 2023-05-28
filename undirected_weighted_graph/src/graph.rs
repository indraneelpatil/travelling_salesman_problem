use plotters::prelude::*;

// Undirected weighted graph
#[derive(Debug)]
pub struct Edge {
    vertex_endpoint_1 : u32,
    vertex_endpoint_2 : u32,
    weight : u64
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