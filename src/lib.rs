// src/lib.rs - Main library module
pub const MAX_VERTICES: usize = 20;

#[derive(Debug, Clone)]
pub struct AdjNode {
    pub vertex: String,
    pub next: Option<Box<AdjNode>>,
}

impl AdjNode {
    pub fn new(vertex: String) -> Self {
        AdjNode {
            vertex,
            next: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Graph {
    pub vertices: Vec<String>,
    pub n_vertices: usize,
    pub adj_matrix: Vec<Vec<i32>>,
    pub adj_list: Vec<Option<Box<AdjNode>>>,
    pub adj_count: Vec<i32>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            vertices: Vec::with_capacity(MAX_VERTICES),
            n_vertices: 0,
            adj_matrix: vec![vec![0; MAX_VERTICES]; MAX_VERTICES],
            adj_list: vec![None; MAX_VERTICES],
            adj_count: vec![0; MAX_VERTICES],
        }
    }
}

// Include the implementation modules
pub mod graph_impl;
pub mod subgraph_impl;
