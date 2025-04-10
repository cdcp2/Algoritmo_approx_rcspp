use std::{cmp::Reverse, collections::BinaryHeap};

/// Implementación del enfoque multi-objetivo para RCSPP
/// Utiliza una ponderación entre costo y recurso que se va ajustando

use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, PartialEq)]
struct F64(f64);

impl Eq for F64 {}

impl PartialOrd for F64 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Usamos total_cmp para definir un orden total
        Some(self.0.total_cmp(&other.0))
    }
}

impl Ord for F64 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
}

pub fn mult_obj(graph: &Vec<Vec<(usize, u32, u32)>>, s: usize, e: usize, resource_limit: u32, inc: f64) -> Option<(Vec<usize>, u32, u32)> {
    let mut lambda = 0.0;
    let mut best_path = None;
    let mut best_cost = u32::MAX;
    let mut best_resource = u32::MAX;

    // Vamos incrementando lambda desde 0 hasta 1
    while lambda <= 1.0 {
        // Definimos una función de costo que combina costo y recurso usando lambda
        let fn_cost = move |(cost, resource): (u32, u32)| -> f64 {
            lambda * (cost as f64) + (1.0 - lambda) * (resource as f64)
        };

        // Ejecutamos Dijkstra con la función de costo actual
        if let Some((path, total_cost, total_resource)) = dijkstra_with_tracking(graph, s, e, fn_cost) {
            // Verificamos si el camino es factible y mejor que el actual
            if total_resource <= resource_limit && (total_cost < best_cost || (total_cost == best_cost && total_resource < best_resource)) {
                best_path = Some(path);
                best_cost = total_cost;
                best_resource = total_resource;
            }
        }

        lambda += inc;
    }

    // Si encontramos un camino factible, lo devolvemos junto con su costo y consumo
    best_path.map(|path| (path, best_cost, best_resource))
}

/// Versión mejorada de Dijkstra que devuelve el camino, costo total y recurso total
fn dijkstra_with_tracking(
    graph: &Vec<Vec<(usize, u32, u32)>>, 
    s: usize, 
    e: usize, 
    fn_cost: impl Fn((u32, u32)) -> f64
) -> Option<(Vec<usize>, u32, u32)> {
    // Cola de prioridad para Dijkstra
    let mut heap: BinaryHeap<Reverse<(F64, usize, u32, u32)>> = BinaryHeap::new();
    
    // Inicializamos con el nodo origen (costo combinado, nodo, costo real, recurso real)
    heap.push(Reverse((F64(0.0), s, 0, 0)));
    
    // Vector de distancias (valor combinado)
    let mut dist = vec![f64::MAX; graph.len()];
    dist[s] = 0.0;
    
    // Para reconstruir el camino
    let mut parent = vec![None; graph.len()];
    
    // Almacenamos el costo y recurso reales para cada nodo
    let mut real_cost = vec![u32::MAX; graph.len()];
    let mut real_resource = vec![u32::MAX; graph.len()];
    real_cost[s] = 0;
    real_resource[s] = 0;

    while let Some(Reverse((f_cost, node, cost, resource))) = heap.pop() {
        // Si ya encontramos un mejor camino a este nodo, ignoramos
        if f_cost > F64(dist[node]) {
            continue;
        }

        // Si llegamos al destino, reconstruimos el camino y lo devolvemos
        if node == e {
            let path = get_path(&parent, e);
            return path.map(|p| (p, cost, resource));
        }

        // Exploramos los vecinos
        for &(next_node, edge_cost, edge_resource) in &graph[node] {
            let next_cost = cost + edge_cost;
            let next_resource = resource + edge_resource;
            
            // Calculamos el costo combinado
            let next_f_cost = fn_cost((next_cost, next_resource));

            // Si encontramos un mejor camino (según la función de costo)
            if next_f_cost < dist[next_node] {
                dist[next_node] = next_f_cost;
                real_cost[next_node] = next_cost;
                real_resource[next_node] = next_resource;
                parent[next_node] = Some(node);
                
                heap.push(Reverse((F64(next_f_cost), next_node, next_cost, next_resource)));
            }
        }
    }

    // Si llegamos aquí, no hay camino al destino
    None
}

/// Reconstruye el camino desde el origen hasta el destino
fn get_path(parent: &Vec<Option<usize>>, mut curr: usize) -> Option<Vec<usize>> {
    let mut path = Vec::new();
    path.push(curr); 
    
    while let Some(node) = parent[curr] {
        path.push(node);
        curr = node;
    }

    path.reverse(); // Invertir el camino para que vaya desde el origen al destino
    Some(path)
}

