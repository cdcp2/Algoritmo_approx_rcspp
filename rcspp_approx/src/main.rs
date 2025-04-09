use crate::genetic_rcsp::Edge;
use std::{collections::HashMap, cmp::Ordering};

use genetic_rcsp::genetic_algorithm;

mod bidirectional_pulse;
mod genetic_rcsp;
mod pulse_algorithm;

fn main() {
    // Ejemplo de un grafo con 5 nodos (0 a 4)
    let mut graph = genetic_rcsp::Graph {
        num_nodes: 5,
        edges: HashMap::new(),
    };
    
    // Definimos las aristas (origen, destino, costo, recursos)
    let edges = vec![
        (0, 1, 2, vec![1, 2]),
        (0, 2, 3, vec![2, 1]),
        (1, 2, 1, vec![7, 1]),
        (1, 3, 4, vec![2, 3]),
        (2, 3, 2, vec![5, 6]),
        (2, 4, 5, vec![3, 1]),
        (3, 4, 1, vec![1, 2]),
    ];
    
    // Agregamos las aristas al grafo
    for (from, to, cost, resources) in edges {
        graph.edges.entry(from).or_insert_with(Vec::new).push(Edge {
            to,
            cost,
            resources,
        });
    }
    
    // Definimos límites de recursos
    let resource_limits = vec![5, 7];
    
    // Ejecutamos el algoritmo genético
    let result = genetic_algorithm(&graph, 50, 100, 0.8, 0.1, &resource_limits);
    
    match result {
        Some((path, cost, consumption)) => println!("Mejor camino encontrado: {:?}, con costo: {:?}, y consumo de: {:?}", path, cost, consumption),
        None => println!("No se encontró un camino válido con las restricciones dadas"),
    }

    

    
}
