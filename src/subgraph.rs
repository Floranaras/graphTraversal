// subgraph.rs - Subgraph detection functionality (equivalent to 6-Bonus.c)
use crate::Graph;
use std::fs::File;
use std::io::{self, Write};

impl Graph {
    /// To remove the file extension from a file's name
    pub fn remove_file_extension(file_name: &str) -> String {
        if let Some(pos) = file_name.rfind('.') {
            file_name[..pos].to_string()
        } else {
            file_name.to_string()
        }
    }

    /// To create the name of the output file with a file extension
    pub fn create_output_filename(base_g: &str, base_h: &str) -> String {
        format!("{}-{}-SUBGRAPH.TXT", base_g, base_h)
    }

    /// To sort an array of integers that correspond to the indices of a graph's vertices
    pub fn sort_vertices_alphabetically(&self) -> Vec<usize> {
        let mut sorted_indices: Vec<usize> = (0..self.n_vertices).collect();

        // Perform Bubble Sort on sorted_indices
        for i in 0..self.n_vertices.saturating_sub(1) {
            for j in 0..self.n_vertices.saturating_sub(1).saturating_sub(i) {
                if self.vertices[sorted_indices[j]] > self.vertices[sorted_indices[j + 1]] {
                    sorted_indices.swap(j, j + 1);
                }
            }
        }

        sorted_indices
    }

    /// To write on a file if a vertex in the second graph exists in the first graph or not
    pub fn write_vertices_status(
        file: &mut File,
        graph_g: &Graph,
        graph_h: &Graph,
        sorted_indices: &[usize],
    ) -> Result<(), Box<dyn std::error::Error>> {
        for &i in sorted_indices {
            // Check if vertex in graph H exists on the graph G
            let vertex_found = graph_g.find_vertex_idx(&graph_h.vertices[i]);
            
            // If vertex exists on both graphs, print +
            if vertex_found.is_some() {
                writeln!(file, "{} +", graph_h.vertices[i])?;
            }
            // If vertex does not exist on both graphs, print -
            else {
                writeln!(file, "{} -", graph_h.vertices[i])?;
            }
        }
        Ok(())
    }

    /// Checks if an edge exists in graph G and returns the appropriate status symbol
    /// Returns '+' if edge exists in graph G, '-' otherwise
    pub fn get_edge_status(&self, graph_h: &Graph, idx1: usize, idx2: usize) -> char {
        // Check if both vertices exist on graph H and G
        let g_index1 = self.find_vertex_idx(&graph_h.vertices[idx1]);
        let g_index2 = self.find_vertex_idx(&graph_h.vertices[idx2]);

        // If either or both vertices cannot be found on both graphs, return -
        if g_index1.is_none() || g_index2.is_none() {
            return '-';
        }

        // If the edge between both vertices does not exist in graph G, return -
        if self.adj_matrix[g_index1.unwrap()][g_index2.unwrap()] != 1 {
            return '-';
        }

        // If both vertices can be found on graphs G and H and
        // the edge connecting both vertices exist in graph G, return +
        '+'
    }

    /// To write on a file if an edge in the second graph exists in the first graph or not
    pub fn write_edges_status(
        file: &mut File,
        graph_g: &Graph,
        graph_h: &Graph,
        sorted_indices: &[usize],
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Loop through all vertices in graph H
        for i in 0..graph_h.n_vertices {
            for j in (i + 1)..graph_h.n_vertices {
                // Determine two vertices
                let idx1 = sorted_indices[i];
                let idx2 = sorted_indices[j];

                // If edge exists on graph H
                if graph_h.adj_matrix[idx1][idx2] == 1 {
                    // Check if edge also exists on graph G
                    let status = graph_g.get_edge_status(graph_h, idx1, idx2);
                    // Then print the corresponding status
                    writeln!(file, "({},{}) {}", graph_h.vertices[idx1], graph_h.vertices[idx2], status)?;
                }
            }
        }
        Ok(())
    }

    /// Searches for a specific vertex in graph G
    /// Returns true if the vertex is found in graph G, false otherwise
    pub fn find_vertex(&self, vertex_name: &str) -> bool {
        // Loop through vertices in graph G
        for vertex in &self.vertices[0..self.n_vertices] {
            // If found, return true
            if vertex == vertex_name {
                return true;
            }
        }
        false
    }

    /// To check if all vertices in graph H exist in graph G
    /// Returns true if all vertices exist, false otherwise
    pub fn all_vertices_exist(&self, graph_h: &Graph) -> bool {
        // Loop through all vertices in graph H
        for i in 0..graph_h.n_vertices {
            // If vertex in graph H cannot be found in graph G
            // return false
            if !self.find_vertex(&graph_h.vertices[i]) {
                return false;
            }
        }
        true
    }

    /// Checks if a specific edge from graph H exists in graph G
    /// Returns true if the edge exists in graph G or no edge exists in graph H, false otherwise
    pub fn check_edge_exists(&self, graph_h: &Graph, h_index1: usize, h_index2: usize) -> bool {
        // If edge does not exist in graph H, return true
        if graph_h.adj_matrix[h_index1][h_index2] != 1 {
            return true;
        }

        // Find indices of the two given vertices
        let g_index1 = self.find_vertex_idx(&graph_h.vertices[h_index1]);
        let g_index2 = self.find_vertex_idx(&graph_h.vertices[h_index2]);

        // If either of the two vertices don't exist on graph G, return false
        if g_index1.is_none() || g_index2.is_none() {
            return false;
        }

        // If edge between two vertices doesn't exist in graph G, return false
        if self.adj_matrix[g_index1.unwrap()][g_index2.unwrap()] == 0 {
            return false;
        }

        // If edge exists in graphs H and G, return true
        true
    }

    /// Determines if all edges in graph H exist in graph G
    /// Returns true if all edges within graph H are also present in graph G, false otherwise
    pub fn all_edges_exist(&self, graph_h: &Graph) -> bool {
        // Loop through all vertices in graph H
        for h_index1 in 0..graph_h.n_vertices {
            // Loop through all vertices in graph H
            for h_index2 in 0..graph_h.n_vertices {
                // If an edge exists between both vertices in graph H
                if graph_h.adj_matrix[h_index1][h_index2] == 1 {
                    // Check if the edge also exists in graph G
                    // If it doesn't exist, return false
                    if !self.check_edge_exists(graph_h, h_index1, h_index2) {
                        return false;
                    }
                }
            }
        }
        true
    }

    /// Determines if graph H is a subgraph of graph G
    pub fn check_is_subgraph(&self, graph_h: &Graph) -> bool {
        // Check if all vertices on graph H exist on graph G
        let all_vertices_found = self.all_vertices_exist(graph_h);
        
        // If all vertices exist on graphs H and G, check if the edges on H also exist on G
        // If some or all vertices on graph H don't exist on graph G, 
        // some or all edges on H cannot be found on G
        let all_edges_found = if all_vertices_found {
            self.all_edges_exist(graph_h)
        } else {
            false
        };

        // If all vertices and edges in graph H exist on graph G, H is a subgraph of G
        all_vertices_found && all_edges_found
    }

    /// To write on a file if a graph is a subgraph of another graph
    pub fn write_subgraph_conclusion(
        file: &mut File,
        base_g: &str,
        base_h: &str,
        is_subgraph: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if is_subgraph {
            writeln!(file, "{} is a subgraph of {}.", base_h, base_g)?;
        } else {
            writeln!(file, "{} is not subgraph of {}.", base_h, base_g)?;
        }
        Ok(())
    }

    /// To create/write the output file
    pub fn produce_subgraph_output(
        &self,
        str_file_g: &str,
        str_file_h: &str,
        graph_h: &Graph,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let str_base_g = Self::remove_file_extension(str_file_g);
        let str_base_h = Self::remove_file_extension(str_file_h);
        let str_output_filename = Self::create_output_filename(&str_base_g, &str_base_h);

        let mut file = File::create(str_output_filename)?;

        // Write output file
        let sorted_indices = graph_h.sort_vertices_alphabetically();
        Self::write_vertices_status(&mut file, self, graph_h, &sorted_indices)?;
        Self::write_edges_status(&mut file, self, graph_h, &sorted_indices)?;

        let is_subgraph = self.check_is_subgraph(graph_h);
        Self::write_subgraph_conclusion(&mut file, &str_base_g, &str_base_h, is_subgraph)?;

        Ok(())
    }

    /// To read the files of both graphs and get their respective details
    /// Returns Ok((graph_g, graph_h)) if both files were successfully read, Err otherwise
    pub fn read_both_graph_files(
        str_file_g: &str,
        str_file_h: &str,
    ) -> Result<(Graph, Graph), Box<dyn std::error::Error>> {
        let mut graph_g = Graph::new();
        let mut graph_h = Graph::new();

        // If either or both files cannot be read, return error
        match graph_g.read_input_file(str_file_g) {
            Ok(_) => {},
            Err(_) => {
                println!("File {} not found.", str_file_g);
                return Err("File not found".into());
            }
        }

        match graph_h.read_input_file(str_file_h) {
            Ok(_) => {},
            Err(_) => {
                println!("File {} not found.", str_file_h);
                return Err("File not found".into());
            }
        }

        Ok((graph_g, graph_h))
    }

    /// To get the file names of the two graphs to be used
    pub fn get_input_filenames() -> Result<(String, String), Box<dyn std::error::Error>> {
        println!("Input first graph filename: ");
        let mut str_file_g = String::new();
        io::stdin().read_line(&mut str_file_g)?;
        let str_file_g = str_file_g.trim().to_string();

        println!("Input second graph filename: ");
        let mut str_file_h = String::new();
        io::stdin().read_line(&mut str_file_h)?;
        let str_file_h = str_file_h.trim().to_string();

        Ok((str_file_g, str_file_h))
    }
}

/// To read the files of two graphs, determine if the program continues,
/// find out if one graph is a subgraph of the other, and produce an output file.
/// Returns Ok(()) on success
pub fn main_subgraph() -> Result<(), Box<dyn std::error::Error>> {
    let (str_file_g, str_file_h) = Graph::get_input_filenames()?;
    
    match Graph::read_both_graph_files(&str_file_g, &str_file_h) {
        Ok((graph_g, graph_h)) => {
            graph_g.produce_subgraph_output(&str_file_g, &str_file_h, &graph_h)?;
            
            // Note: In Rust, we don't need to manually free memory as it's handled automatically
            println!("Subgraph analysis completed successfully!");
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    Ok(())
}
