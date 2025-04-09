use crate::genetic_rcsp::Edge;
use std::collections::HashMap;

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

    println!("\n--- PRUEBA ALGORITMO PULSE ---");
    
    // Crear un grafo de ejemplo con 6 nodos
    // Representación: Vec<Vec<(nodo_destino, costo, consumo_recurso)>>
    let graph: Vec<Vec<(usize, u32, u32)>> = vec![
        // Nodo 0 (origen)
        vec![(1, 2, 3), (2, 3, 1)],
        
        // Nodo 1
        vec![(3, 4, 2), (4, 1, 5)],
        
        // Nodo 2
        vec![(3, 1, 3), (4, 5, 2)],
        
        // Nodo 3
        vec![(5, 3, 2)],
        
        // Nodo 4
        vec![(5, 2, 1)],
        
        // Nodo 5 (destino)
        vec![]
    ];
    
    // Parámetros del problema
    let source = 5;
    let target = 4;
    
    // Visualizar el grafo
    println!("Grafo de prueba:");
    for (i, edges) in graph.iter().enumerate() {
        print!("Nodo {}: ", i);
        for (dest, cost, resource) in edges {
            print!("→ {}(c:{},r:{}) ", dest, cost, resource);
        }
        println!();
    }
    
    // Caminos posibles en este grafo:
    println!("\nCaminos posibles (origen → destino):");
    println!("0 → 1 → 3 → 5: Costo total = 9, Consumo total = 7");
    println!("0 → 1 → 4 → 5: Costo total = 5, Consumo total = 9");
    println!("0 → 2 → 3 → 5: Costo total = 7, Consumo total = 6");
    println!("0 → 2 → 4 → 5: Costo total = 10, Consumo total = 4");
    
    // Probar con diferentes límites de recursos
    let resource_limits = vec![4, 6, 8, 10];
    
    for &limit in &resource_limits {
        println!("\nPrueba con límite de recursos = {}", limit);
        
        let result = pulse_algorithm::pulse_algorithm(graph.clone(), source, target, limit);
        
        match result {
            Some(pulse) => {
                println!(" Camino encontrado: {:?}", pulse.path);
                println!("   Costo total: {}", pulse.cost);
                println!("   Consumo de recursos: {}", pulse.consumption);
            },
            None => {
                println!(" No se encontró un camino válido con el límite de recursos dado");
            }
        }
    }

    
}
