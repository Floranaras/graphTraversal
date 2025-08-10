// src/graph_impl.rs - Core graph implementation (equivalent to graph.c)
use crate::{Graph, AdjNode, MAX_VERTICES};
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

impl Graph {
    /// Gets the name of the file without extension from user input and copy into base_name
    pub fn get_base_filename(input_name: &str) -> String {
        if let Some(pos) = input_name.rfind('.') {
            input_name[..pos].to_string()
        } else {
            input_name.to_string()
        }
    }

    /// Initialize the adjacency representations of the graph data structure
    pub fn init_rep(&mut self) {
        for i in 0..MAX_VERTICES {
            self.adj_count[i] = 0;
            for j in 0..MAX_VERTICES {
                self.adj_matrix[i][j] = 0;
            }
        }
        
        for i in 0..MAX_VERTICES {
            self.adj_list[i] = None;
        }
    }

    /// Creates a new node and set vertex as label of new node
    pub fn create_node(vertex: String) -> Box<AdjNode> {
        Box::new(AdjNode::new(vertex))
    }

    /// Adds a new node into the adjacency list representation
    pub fn add_to_adj_list(&mut self, vertex_idx: usize, adj_vertex: String) {
        let new_node = Self::create_node(adj_vertex);
        
        if self.adj_list[vertex_idx].is_none() {
            self.adj_list[vertex_idx] = Some(new_node);
        } else {
            let mut current = &mut self.adj_list[vertex_idx];
            while let Some(ref mut node) = current {
                if node.next.is_none() {
                    node.next = Some(new_node);
                    break;
                }
                current = &mut node.next;
            }
        }
        
        self.adj_count[vertex_idx] += 1;
    }

    /// Removes every node in the adjacency list
    pub fn free_adj_list(&mut self) {
        for i in 0..MAX_VERTICES {
            self.adj_list[i] = None;
            self.adj_count[i] = 0;
        }
    }

    /// Searches through the known vertex labels in the graph and return the index of the target label
    pub fn find_vertex_idx(&self, str_vertex: &str) -> Option<usize> {
        for (i, vertex) in self.vertices.iter().enumerate() {
            if vertex == str_vertex {
                return Some(i);
            }
        }
        None
    }

    /// Creates the adjacency matrix representation of a graph
    pub fn make_adj_matrix(&mut self) {
        // Initialize all values in adjacency matrix to 0
        for i in 0..MAX_VERTICES {
            for j in 0..MAX_VERTICES {
                self.adj_matrix[i][j] = 0;
            }
        }

        // Loop through all vertices
        for i in 0..self.n_vertices {
            let mut current = &self.adj_list[i];
            // Loop through all neighbors of a vertex
            while let Some(ref node) = current {
                // Find index of vertex's neighbor on adjacency list
                if let Some(adj_idx) = self.find_vertex_idx(&node.vertex) {
                    // If index was found (edge exists between both vertex and neighbor),
                    // position on matrix is set to 1
                    self.adj_matrix[i][adj_idx] = 1;
                }
                // Move to next neighbor of the vertex
                current = &node.next;
            }
        }
    }

    /// Reads information from input file and add to the graph data structure
    pub fn read_input_file(&mut self, str_input_filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open(str_input_filename)?;
        let mut reader = BufReader::new(file);
        let mut line = String::new();

        // Read number of vertices
        reader.read_line(&mut line)?;
        self.n_vertices = line.trim().parse()?;

        // Initialize adjacency matrix representation
        self.init_rep();

        // Read adjacency info from file and create adjacency list
        for i in 0..self.n_vertices {
            line.clear();
            reader.read_line(&mut line)?;
            let parts: Vec<&str> = line.trim().split_whitespace().collect();
            
            if parts.is_empty() {
                continue;
            }

            self.vertices.push(parts[0].to_string());
            self.adj_count[i] = 0;

            // Read adjacent vertices until -1
            for j in 1..parts.len() {
                if parts[j] == "-1" {
                    break;
                }
                self.add_to_adj_list(i, parts[j].to_string());
            }
        }

        self.make_adj_matrix();
        Ok(())
    }

    /// Sets the name of the output text file
    pub fn get_output_filename(base_name: &str, suffix: &str) -> String {
        format!("{}{}.TXT", base_name, suffix)
    }

    /// Sorts the index of the vertices in the graph by value of the label of each node into idx
    pub fn sort_vertices(&self) -> Vec<usize> {
        let mut idx: Vec<usize> = (0..self.n_vertices).collect();

        // Perform Bubble Sort on idx
        for i in 0..self.n_vertices.saturating_sub(1) {
            for j in 0..self.n_vertices.saturating_sub(1).saturating_sub(i) {
                if self.vertices[idx[j]] > self.vertices[idx[j + 1]] {
                    idx.swap(j, j + 1);
                }
            }
        }

        idx
    }

    /// To sort an array of integers that correspond to the indices of a graph's vertices (alias for sort_vertices)
    pub fn sort_vertices_alphabetically(&self) -> Vec<usize> {
        self.sort_vertices()
    }

    /// Prepares the output file of list of vertices and edges in the graph
    pub fn produce_output_file1(&self, base_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let output_name = Self::get_output_filename(base_name, "-SET");
        let mut fp = File::create(output_name)?;
        
        let sorted_idx = self.sort_vertices();

        // Write vertices of graph
        write!(fp, "V({})={{", base_name)?;
        for (i, &idx) in sorted_idx.iter().enumerate() {
            write!(fp, "{}", self.vertices[idx])?;
            if i < self.n_vertices - 1 {
                write!(fp, ",")?;
            }
        }
        writeln!(fp, "}}")?;

        // Write edges of graph
        write!(fp, "E({})={{", base_name)?;
        let mut edge_ctr = 0;
        for i in 0..self.n_vertices {
            for j in (i + 1)..self.n_vertices {
                if self.adj_matrix[sorted_idx[i]][sorted_idx[j]] == 1 {
                    if edge_ctr > 0 {
                        write!(fp, ",")?;
                    }
                    write!(fp, "({},{})", self.vertices[sorted_idx[i]], self.vertices[sorted_idx[j]])?;
                    edge_ctr += 1;
                }
            }
        }
        writeln!(fp, "}}")?;

        Ok(())
    }

    /// Finds the adjacent vertices of a node and add into adj_vertices
    pub fn collect_adjacent_vertices(&self, adj_list: &Option<Box<AdjNode>>) -> Vec<String> {
        let mut adj_vertices = Vec::new();
        let mut current = adj_list;
        
        // Look through all neighbors of the vertex
        while let Some(ref node) = current {
            // Copy the vertex name of the neighbor into array
            adj_vertices.push(node.vertex.clone());
            current = &node.next;
        }
        
        adj_vertices
    }

    /// Print the adjacency list of a node
    pub fn print_vertex_adjacency_list(
        fp: &mut File,
        vertex: &str,
        adj_vertices: &[String],
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Print first vertex
        write!(fp, "{}->", vertex)?;
        
        // Print all vertices with an edge to the first vertex
        for adj_vertex in adj_vertices {
            write!(fp, "{}->", adj_vertex)?;
        }
        
        write!(fp, "\\")?;
        Ok(())
    }

    /// To create/write an output file that lists a graph's vertices and their degrees
    pub fn produce_output_file2(&self, base_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Get file name of output file
        let output_name = Self::get_output_filename(base_name, "-DEGREE");
        
        // Open output file
        let mut fp = File::create(output_name)?;
        
        // Get indices of sorted vertex
        let sorted_idx = self.sort_vertices();

        // Print vertices in ascending order along with their degrees
        for (i, &idx) in sorted_idx.iter().enumerate() {
            write!(fp, "{:<10}{}", self.vertices[idx], self.adj_count[idx])?;
            if i < self.n_vertices - 1 {
                writeln!(fp)?;
            }
        }

        Ok(())
    }

    /// To create/write an output file that visualizes an adjacency list representation of a graph
    pub fn produce_output_file3(&self, base_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Get file name of output file
        let output_name = Self::get_output_filename(base_name, "-LIST");
        
        // Open output file
        let mut fp = File::create(output_name)?;

        // Print vertices in their original input order (no sorting of vertices)
        for i in 0..self.n_vertices {
            // Collect all adjacent vertices from linked list
            let adj_vertices = self.collect_adjacent_vertices(&self.adj_list[i]);
            
            // Print adjacency list for this vertex
            Self::print_vertex_adjacency_list(&mut fp, &self.vertices[i], &adj_vertices)?;
            writeln!(fp)?;
        }

        Ok(())
    }

    /// To create/write an output file that visualizes an adjacency matrix representation of a graph
    pub fn produce_output_file4(&self, base_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let output_name = Self::get_output_filename(base_name, "-MATRIX");
        let mut fp = File::create(output_name)?;

        write!(fp, "{:<10}", "")?;
        
        // Print column vertices of matrix
        for i in 0..self.n_vertices {
            write!(fp, "{:<10}", self.vertices[i])?;
        }
        writeln!(fp)?;

        // Print rows of matrix
        for i in 0..self.n_vertices {
            // Print row vertex
            write!(fp, "{:<10}", self.vertices[i])?;
            
            // Prints 1 if row vertex has an edge with column vertex, 0 otherwise
            for j in 0..self.n_vertices {
                write!(fp, "{:<10}", self.adj_matrix[i][j])?;
            }
            writeln!(fp)?;
        }

        Ok(())
    }

    /// To traverse a graph at a given index using the Breadth First Search Algorithm (BFS)
    /// then storing the results in an array
    pub fn bfs(&self, starting_index: usize) -> Vec<String> {
        let mut visited = vec![false; self.n_vertices];
        let mut queue = VecDeque::new();
        let mut result = Vec::new();

        // Put starting index in first index of queue
        queue.push_back(starting_index);
        // Visit starting index
        visited[starting_index] = true;

        // Loop runs until queue is empty
        while let Some(current_vertex) = queue.pop_front() {
            // Insert the frontmost, unvisited vertex stored in the queue
            result.push(self.vertices[current_vertex].clone());

            // Reset candidates counter
            // Then find candidates (neighbors of current vertex that have not been visited)
            let mut candidates = Vec::new();
            for i in 0..self.n_vertices {
                if self.adj_matrix[current_vertex][i] == 1 && !visited[i] {
                    candidates.push(i);
                    visited[i] = true;
                }
            }

            // Perform Selection Sort on candidates array to arrange candidates in alphabetical order
            candidates.sort_by(|&a, &b| self.vertices[a].cmp(&self.vertices[b]));

            // Enqueue all candidates and repeat the loop until queue is empty
            for candidate in candidates {
                queue.push_back(candidate);
            }
        }

        result
    }

    /// To traverse a graph at a given index using the Depth First Search Algorithm (DFS)
    /// then storing the results in an array
    pub fn dfs(&self, previous_index: usize, result: &mut Vec<String>, visited: &mut Vec<bool>) {
        // Visit indicated vertex with index of previous_index
        // On first call of DFS, previous index is the starting index
        visited[previous_index] = true;
        result.push(self.vertices[previous_index].clone());

        // Find candidates (neighbors of recently visited vertex that have not been visited yet)
        let mut candidates = Vec::new();
        for i in 0..self.n_vertices {
            if self.adj_matrix[previous_index][i] == 1 && !visited[i] {
                candidates.push(i);
            }
        }

        // Perform Selection Sort on candidates array to arrange candidates in alphabetical order
        candidates.sort_by(|&a, &b| self.vertices[a].cmp(&self.vertices[b]));

        // Visit all candidates of recently visited vertex
        for candidate in candidates {
            if !visited[candidate] {
                self.dfs(candidate, result, visited);
            }
        }
    }

    /// To create/write an output file that prints the traversal order of a graph using 
    /// Breadth First Search (BFS)
    pub fn produce_output_file5(&self, base_name: &str, start: &str) -> Result<(), Box<dyn std::error::Error>> {
        let output_name = Self::get_output_filename(base_name, "-BFS");
        
        // Determine if starting vertex exists on the graph
        let starting_idx = self.find_vertex_idx(start);
        
        let mut fp = File::create(output_name)?;

        if let Some(starting_idx) = starting_idx {
            // Perform BFS
            let result = self.bfs(starting_idx);
            
            // Then print results
            for (i, vertex) in result.iter().enumerate() {
                write!(fp, "{}", vertex)?;
                if i < result.len() - 1 {
                    write!(fp, " ")?;
                }
            }
            writeln!(fp)?;
        } else {
            return Err("Starting vertex not found".into());
        }

        Ok(())
    }

    /// To create/write an output file that prints the traversal order of a graph using 
    /// Depth First Search (DFS)
    pub fn produce_output_file6(&self, base_name: &str, start: &str) -> Result<(), Box<dyn std::error::Error>> {
        let output_name = Self::get_output_filename(base_name, "-DFS");
        
        // Check if starting vertex exists on graph
        let starting_idx = self.find_vertex_idx(start);
        
        let mut fp = File::create(output_name)?;

        if let Some(starting_idx) = starting_idx {
            // Initialize visited array
            let mut visited = vec![false; MAX_VERTICES];
            let mut result = Vec::new();
            
            // Perform DFS
            self.dfs(starting_idx, &mut result, &mut visited);
            
            // Then print results
            for (i, vertex) in result.iter().enumerate() {
                write!(fp, "{}", vertex)?;
                if i < result.len() - 1 {
                    write!(fp, " ")?;
                }
            }
            writeln!(fp)?;
        } else {
            return Err("Starting vertex not found".into());
        }

        Ok(())
    }
}
