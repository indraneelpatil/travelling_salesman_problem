// Created by Indraneel on 29/05/23
use std::collections::HashMap;
use rand::Rng;
use graph_lib::*;

pub fn prims_solver(graph : &mut Graph) {

    let mut dist_to_t = HashMap::new();
    let mut vertex_parent = HashMap::new();

    // Initialize all distances to infinity
    let mut vertex_count = 0;
    for _vertex in &graph.vertices {
        dist_to_t.insert(vertex_count, std::f64::INFINITY);
        vertex_parent.insert(vertex_count, -1);
        vertex_count+=1;
    }

    // Get random vertex
    let mut rng = rand::thread_rng();
    let mut closest_vert_ind = rng.gen_range(0..=graph.vertices.len() as i32);

    while closest_vert_ind != -1 {

        //println!("Closest vertex index: {}", closest_vert_ind);
        // Add closest vertex to tree
        dist_to_t.insert(closest_vert_ind, std::f64::NEG_INFINITY);
        let parent_edge_ind = vertex_parent[&closest_vert_ind];
        if parent_edge_ind != -1 {
            graph.edges[parent_edge_ind as usize].on_mst = true;
        } 

        // Get neighbours of closest vertex
        let mut edge_ind = 0;
        for edge in &graph.edges {
            let mut neighbour_ind = -1;
            if edge.endpoint1 == closest_vert_ind {
                neighbour_ind = edge.endpoint2;
            }
            else if edge.endpoint2 == closest_vert_ind {
                neighbour_ind = edge.endpoint1;
            }

            if neighbour_ind != -1 {
                //println!("Neighbour index: {} my index {} edge {}", neighbour_ind, closest_vert_ind, edge_ind);
                // Update distance to tree and vertex parent if necessary
                if dist_to_t[&neighbour_ind] > edge.weight {
                    dist_to_t.insert(neighbour_ind, edge.weight);
                    vertex_parent.insert(neighbour_ind, edge_ind);
                }
            }
            edge_ind += 1;
        }

        // Get closest vertex to tree
        let mut closest_dist = std::f64::INFINITY;
        closest_vert_ind = -1;
        for (vertex_ind, dist) in &dist_to_t {
            // Check if it is the closest vertex and not already on the tree
            if *dist < closest_dist && *dist != std::f64::NEG_INFINITY {
                closest_dist = *dist;
                closest_vert_ind = *vertex_ind;
            }
        }
    }

}
