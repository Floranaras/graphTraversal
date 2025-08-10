use std::io;
use graph_analysis::{Graph, subgraph_impl::main_subgraph};

fn main_graph_traversal() -> Result<(), Box<dyn std::error::Error>> {
    println!("Input filename: ");
    let mut str_input_filename = String::new();
    io::stdin().read_line(&mut str_input_filename)?;
    let str_input_filename = str_input_filename.trim();

    let mut graph = Graph::new();
    
    match graph.read_input_file(str_input_filename) {
        Ok(_) => {
            println!("Input starting label: ");
            let mut str_start_vertex = String::new();
            io::stdin().read_line(&mut str_start_vertex)?;
            let str_start_vertex = str_start_vertex.trim();

            let str_base_filename = Graph::get_base_filename(str_input_filename);

            graph.produce_output_file1(&str_base_filename)?;
            graph.produce_output_file2(&str_base_filename)?;
            graph.produce_output_file3(&str_base_filename)?;
            graph.produce_output_file4(&str_base_filename)?;

            if graph.find_vertex_idx(str_start_vertex).is_some() {
                graph.produce_output_file5(&str_base_filename, str_start_vertex)?;
                graph.produce_output_file6(&str_base_filename, str_start_vertex)?;
            }

            graph.free_adj_list();
            println!("All output files generated successfully!");
        }
        Err(_) => {
            println!("File {} not found.", str_input_filename);
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Graph Analysis Program");
    println!("1. Graph Traversal");
    println!("2. Subgraph Detection");
    println!("Select option (1 or 2): ");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice)?;
    
    match choice.trim() {
        "1" => main_graph_traversal(),
        "2" => main_subgraph(),
        _ => {
            println!("Invalid choice. Running graph traversal by default.");
            main_graph_traversal()
        }
    }
}
