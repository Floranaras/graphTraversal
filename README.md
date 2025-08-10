# Graph Analysis Library in Rust

A comprehensive Rust implementation of graph data structures with traversal algorithms and subgraph detection capabilities.

## Features

- **Graph Representations**: Both adjacency matrix and adjacency list implementations
- **Graph Traversals**: Breadth-First Search (BFS) and Depth-First Search (DFS) algorithms
- **File I/O**: Read graph data from text files and generate multiple output formats
- **Subgraph Detection**: Determine if one graph is a subgraph of another
- **Multiple Output Formats**: 
  - Vertex and edge sets
  - Degree information
  - Adjacency list visualization
  - Adjacency matrix visualization
  - Traversal results

## Project Structure

```
src/
├── lib.rs              # Library definitions and main structures
├── graph_impl.rs       # Core graph implementation
├── subgraph_impl.rs    # Subgraph detection functionality
└── main.rs             # Main driver program
Cargo.toml              # Project configuration
README.md               # This file
```

## Module Overview

| Rust File | Purpose |
|-----------|---------|
| `lib.rs` | Type definitions and module declarations |
| `graph_impl.rs` | Core graph implementation and algorithms |
| `subgraph_impl.rs` | Subgraph detection functionality |
| `main.rs` | Main program driver and CLI interface |

## Installation and Setup

### Prerequisites

- Rust 1.70.0 or later
- Cargo package manager (included with Rust)

### Building the Project

```bash
# Clone the repository
git clone <repository-url>
cd graph_analysis

# Build the project
cargo build

# Build with optimizations
cargo build --release
```

### Running the Program

```bash
# Run the main program
cargo run

# Or run the compiled binary
./target/release/main
```

## Usage

The program offers two main modes of operation:

### 1. Graph Traversal Mode

Performs comprehensive graph analysis and generates multiple output files:

1. Input a graph filename (must exist in the project directory)
2. Input a starting vertex label for traversals
3. The program generates 6 output files:
   - `{filename}-SET.TXT`: Vertex and edge sets
   - `{filename}-DEGREE.TXT`: Vertex degrees
   - `{filename}-LIST.TXT`: Adjacency list representation
   - `{filename}-MATRIX.TXT`: Adjacency matrix representation
   - `{filename}-BFS.TXT`: BFS traversal results
   - `{filename}-DFS.TXT`: DFS traversal results

### 2. Subgraph Detection Mode

Analyzes the relationship between two graphs:

1. Input first graph filename (potential supergraph)
2. Input second graph filename (potential subgraph)
3. The program generates one output file:
   - `{graph1}-{graph2}-SUBGRAPH.TXT`: Subgraph analysis results

## Input File Format

Graph input files should follow this format:

```
<number_of_vertices>
<vertex1> <adjacent_vertex1> <adjacent_vertex2> ... -1
<vertex2> <adjacent_vertex1> <adjacent_vertex2> ... -1
...
```

### Example Input File

```
3
A B C -1
B A -1
C A -1
```

This represents a graph with vertices A, B, C where:
- A is connected to B and C
- B is connected to A
- C is connected to A

## Testing

Run the comprehensive test suite:

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_graph_creation

# Run tests with optimizations
cargo test --release
```

## Development

### Code Organization

The codebase is organized into modules for clear separation of concerns:

- **lib.rs**: Contains the main data structures (`Graph`, `AdjNode`) and constants
- **graph_impl.rs**: Implements core graph operations, file I/O, and traversal algorithms
- **subgraph_impl.rs**: Implements subgraph detection and comparison functionality
- **main.rs**: Provides the command-line interface and program entry point

### Key Data Structures

```rust
pub struct Graph {
    pub vertices: Vec<String>,
    pub n_vertices: usize,
    pub adj_matrix: Vec<Vec<i32>>,
    pub adj_list: Vec<Option<Box<AdjNode>>>,
    pub adj_count: Vec<i32>,
}

pub struct AdjNode {
    pub vertex: String,
    pub next: Option<Box<AdjNode>>,
}
```

### Algorithms Implemented

- **Breadth-First Search (BFS)**: Level-order graph traversal
- **Depth-First Search (DFS)**: Recursive depth-first traversal
- **Subgraph Detection**: Vertex and edge matching algorithm
- **Graph Sorting**: Alphabetical vertex ordering

## Performance Considerations

- **Debug Mode**: Use `cargo run` for development with debug symbols
- **Release Mode**: Use `cargo run --release` for optimized performance
- **Memory Usage**: Rust's zero-cost abstractions provide excellent performance
- **Safety**: Memory safety guaranteed at compile time

## Language Benefits

This Rust implementation provides several advantages:

1. **Memory Safety**: Automatic memory management prevents leaks and corruption
2. **Error Handling**: Comprehensive error handling using `Result<T, E>`
3. **Type Safety**: Strong typing prevents many runtime errors
4. **Modern Collections**: Efficient use of `Vec`, `VecDeque`, and other collections
5. **String Handling**: Safe and efficient string operations
6. **Modularity**: Clean separation of concerns with Rust's module system

## Dependencies

This implementation uses only Rust's standard library with no external dependencies for the core functionality.

## Commit Message Guidelines

This project uses Conventional Commits for clear and standardized commit messages:

| **Type** | **Purpose** |
|----------|-------------|
| `feat` | Add a new feature (functions, logic) |
| `fix` | Fix a bug (incorrect output, logic errors) |
| `refactor` | Improve code without changing behavior |
| `perf` | Optimize performance (faster loops, better memory usage) |
| `style` | Formatting changes (indentation, comments) |
| `test` | Add or update test cases |
| `build` | Modify Cargo.toml or compilation setup |
| `docs` | Update README, specs, or comments |
| `chore` | Non-code maintenance (renaming files, updating `.gitignore`) |

### Example Commit Messages

```
feat: implement BFS traversal algorithm
fix: correct adjacency matrix initialization
refactor: extract file reading logic into separate function
perf: optimize vertex sorting with better algorithm
test: add comprehensive subgraph detection tests
docs: update README with usage examples
```

## Contributing

1. Follow Rust naming conventions (`snake_case` for functions, `PascalCase` for types)
2. Add comprehensive tests for new functionality
3. Update documentation for public APIs
4. Run `cargo fmt` and `cargo clippy` before submitting
5. Use conventional commit messages as outlined above
6. Ensure all tests pass with `cargo test`

## Troubleshooting

### Common Issues

1. **File Not Found**: Ensure input files are in the project root directory
2. **Compilation Errors**: Run `cargo clean` then `cargo build`
3. **Test Failures**: Check that input format matches specification

### Debug Mode

For debugging, you can add print statements or use the Rust debugger:

```bash
# Run with debug output
RUST_LOG=debug cargo run

# Use the Rust debugger
rust-gdb target/debug/main
```

## License

This project is licensed under the MIT License.

## Authors

- Graph Analysis Development Team

## Version History

- **v0.1.0**: Initial release with full graph analysis functionality

## Future Enhancements

Potential improvements for future versions:
- Parallel graph algorithms
- Graph visualization output
- Additional graph algorithms (shortest path, minimum spanning tree)
- JSON input/output format support
- Performance benchmarking suite
