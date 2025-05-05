use std::{cmp::Reverse, collections::BinaryHeap};

const KAPPA: u32 = 2;
const P_MAX: u32 = 1000000;

pub fn edge_penalization(graph: &Vec<Vec<(usize, u32, u32)>>, s:usize, e:usize, resource_limit: u32)-> Option<(Vec<usize>, u32, u32)> {
    let mut graph_ext: Vec<Vec<(usize, u32, u32, u32)>> = graph
    .iter()
    .map(|adj| {
        adj.iter()
            .map(|&(dst, cost, res)| (dst, cost, res, 1)) // añade el flag en false
            .collect()
    })
    .collect();
    let mut best: Option<(Vec<usize>, u32, u32)> = None;

    
    
    while let Some(path) = dijkstra(&graph_ext, s, e) {
        penalize_or_block_heaviest_edge(&mut graph_ext, &path.0);
        if path.2 <= resource_limit                      
            && best.as_ref().map_or(true, |b| path.1 < b.1) { 
                best = Some(path);                            
        } else if path.2 > resource_limit{break;}
    }

    
    best
}

fn dijkstra(graph: &Vec<Vec<(usize,u32,u32,u32)>>, s:usize, e:usize)-> Option<(Vec<usize>, u32, u32)> {
    // Cola de prioridad para Dijkstra
    let mut heap: BinaryHeap<Reverse<(u32, usize, u32,u32)>> = BinaryHeap::new();
    
    // Inicializamos con el nodo origen (costo, nodo)
    heap.push(Reverse((0, s,0,0)));
    
    let mut dist = vec![u32::MAX; graph.len()];
    dist[s] = 0;
    
    // Para reconstruir el camino
    let mut parent: Vec<Option<usize>> = vec![None; graph.len()];
    

    while let Some(Reverse((f_cost, node, accum_cost, accum_resource))) = heap.pop() {
        // Si ya encontramos un mejor camino a este nodo, ignoramos
        if f_cost > dist[node] {
            continue;
        }

        // Si llegamos al destino, reconstruimos el camino y lo devolvemos
        if node == e {
            let path = get_path(parent, e);
            return path.map(|p| (p, accum_cost, accum_resource));
        }

        // Exploramos los vecinos
        for &(next_node, edge_cost, edge_resource, pen) in &graph[node] {
            let next_cost = accum_cost + edge_cost;
            let next_resource = accum_resource + edge_resource;
            
            

            // Si encontramos un mejor camino (según la función de costo)
            if next_resource < dist[next_node] && pen < P_MAX{
                dist[next_node] = next_resource;
                
                parent[next_node] = Some(node);
                
                heap.push(Reverse(((next_resource.saturating_mul(pen)), next_node, next_cost, next_resource)));
            }
        }
    }

    // Si llegamos aquí, no hay camino al destino
    None
}

fn get_path(parent: Vec<Option<usize>>, mut curr: usize) -> Option<Vec<usize>> {
    let mut path = Vec::new();
    path.push(curr); 
    
    while let Some(node) = parent[curr] {
        path.push(node);
        curr = node;
    }

    path.reverse(); // Invertir el camino para que vaya desde el origen al destino
    Some(path)
}

/// Marca como bloqueado únicamente el arco de costo más alto de `path`.
fn penalize_or_block_heaviest_edge(
    graph: &mut Vec<Vec<(usize, u32, u32, u32)>>,
    path: &[usize],
) {
    // 1.  Buscar el (u,v) con mayor costo dentro del camino.
    let mut max_cost = 0;
    let mut max_from = None;
    let mut max_to   = None;
    let mut consumed = 0;

    for uv in path.windows(2) {
        let (u, v) = (uv[0], uv[1]);

        if let Some(&(dst, cost, resource, _)) =
            graph[u].iter().find(|e| e.0 == v)
        {
            if cost > max_cost || cost == max_cost && resource > consumed {
                max_cost = cost;
                consumed = resource;
                max_from = Some(u);
                max_to   = Some(dst);
            }
        }
    }

    // 2.  Bloquear solo ese arco.
    if let (Some(u), Some(v)) = (max_from, max_to) {
        if let Some(e) = graph[u].iter_mut().find(|e| e.0 == v) {
            e.3 *= KAPPA;
        }
    }
}
