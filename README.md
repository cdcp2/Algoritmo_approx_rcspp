# Approximation Algorithms for the Resource-Constrained Shortest Path Problem

Rust implementation and experimental comparison of algorithms for the **Resource-Constrained Shortest Path Problem (RCSPP)**.

Given a directed graph, a source, a destination, and a resource limit, the program searches for a feasible path that minimizes cost without exceeding the resource constraint. The project compares an exact reference method with several approximation strategies on large road-network datasets.

## Implemented approaches

- **Pulse algorithm:** exact search with backtracking, lower bounds, dominance checks, and pruning.
- **Multi-objective search:** repeated Dijkstra searches with different cost-resource weightings.
- **Disjoint-path exploration:** generates alternatives by blocking previously selected paths.
- **Edge blocking:** blocks the most expensive edge of a candidate path to explore alternatives.
- **Edge penalization:** progressively penalizes selected edges instead of immediately removing them.

The Pulse algorithm runs with a timeout and provides a reference cost when it completes. The program reports execution time and the approximation ratio of the other approaches relative to that reference.

## Technical highlights

- Rust 2021 and Cargo
- Adjacency-list graph representation
- Priority queues with `BinaryHeap`
- Dijkstra-based search
- Backtracking and pruning
- Resource-feasibility checks
- Threads, channels, and shared graph data with `Arc`
- Runtime benchmarking on large graph instances
- Python scripts for batch execution and CSV result collection

## Repository structure

```text
.
├── rcspp_approx/
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs
│   │   ├── pulse_algorithm.rs
│   │   ├── mult_obj_approach.rs
│   │   ├── disjoint_path_approach.rs
│   │   ├── edge_blocking_algo.rs
│   │   └── edge_penalization.rs
│   ├── Instancias/
│   └── runner.py
├── Resultados/
└── *_graph_*.txt
```

## Requirements

- Rust toolchain with Cargo
- Python 3 with pandas, only for the optional batch-processing scripts

Install Rust through [rustup](https://rustup.rs/) if it is not already available.

## Build

```bash
cd rcspp_approx
cargo build --release
```

## Run

```bash
cargo run --release -- <graph_file> <source_node> <target_node> <resource_limit>
```

Example using one of the included datasets:

```bash
cargo run --release -- ../NY_graph_dist_time.txt <source_node> <target_node> <resource_limit>
```

Each non-empty input line must have four whitespace-separated values:

```text
from_node to_node cost resource_consumption
```

Node identifiers must be non-negative integers. Cost, resource consumption, and the resource limit are represented as unsigned integers.

## Output

For each approach, the program reports:

- Selected path
- Total path cost
- Total resource consumption
- Execution time
- Approximation ratio against the Pulse reference result, when available

## Background

RCSPP is a classical combinatorial optimization problem. It extends shortest-path search by adding one or more resource constraints and appears in routing, scheduling, logistics, transportation, and network optimization.

This repository was developed as an academic algorithms project in 2025.
