use std::{cmp::Reverse, collections::BinaryHeap};


fn dijkstra<F>(graph: &Vec<Vec<(usize,u32,u32,bool)>>, s:usize, e:usize, fn_cost: F,) -> Option<(Vec<usize>, u32, u32)>
    where
        F: Fn(u32,u32)->u32, 
    {
    // Cola de prioridad para Dijkstra
    let mut heap: BinaryHeap<Reverse<(u32, usize, u32,u32)>> = BinaryHeap::new();
    
    // Inicializamos con el nodo origen (costo, nodo)
    heap.push(Reverse((0, s,0,0)));
    
    let mut dist = vec![u32::MAX; graph.len()];
    dist[s] = 0;
    
    // Para reconstruir el camino
    let mut parent = vec![None; graph.len()];
    

    while let Some(Reverse((f_cost, node, accum_cost, accum_resource))) = heap.pop() {
        // Si ya encontramos un mejor camino a este nodo, ignoramos
        if f_cost > dist[node] {
            continue;
        }

        // Si llegamos al destino, reconstruimos el camino y lo devolvemos
        if node == e {
            let path = get_path(&parent, e);
            return path.map(|p| (p, accum_cost, accum_resource));
        }

        // Exploramos los vecinos
        for &(next_node, edge_cost, edge_resource, block) in &graph[node] {
            let next_cost = accum_cost + edge_cost;
            let next_resource = accum_resource + edge_resource;
            
            // Calculamos el costo combinado
            let next_f_cost = fn_cost(next_cost, next_resource);

            // Si encontramos un mejor camino (según la función de costo)
            if next_f_cost < dist[next_node] && !block{
                dist[next_node] = next_f_cost;
                
                parent[next_node] = Some(node);
                
                heap.push(Reverse((next_cost, next_node, next_cost, next_resource)));
            }
        }
    }

    // Si llegamos aquí, no hay camino al destino
    None
}

pub fn disjoint_algo(graph: &Vec<Vec<(usize, u32, u32)>>, s:usize, e:usize, resource_limit: u32)-> Option<(Vec<usize>, u32, u32)> {
    let mut graph_ext: Vec<Vec<(usize, u32, u32, bool)>> = graph
    .iter()
    .map(|adj| {
        adj.iter()
            .map(|&(dst, cost, res)| (dst, cost, res, false)) // añade el flag en false
            .collect()
    })
    .collect();
    let mut best: Option<(Vec<usize>, u32, u32)> =None;

    for cost in 1..=2 {
        let fn_cost = move |c1:u32, c2:u32|->u32{if cost == 1 {c1}else{c2}};
        while let Some(path) = dijkstra(&graph_ext, s, e, fn_cost) {
            block_path(&mut graph_ext, &path.0);
            if path.2 <= resource_limit                      
            && best.as_ref().map_or(true, |b| path.1 < b.1) { 
                best = Some(path);                            
            }
        }
        unblock_network(&mut graph_ext);

    }
    best
}


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

fn block_path(graph: &mut Vec<Vec<(usize,u32,u32,bool)>>, path: &Vec<usize>) {
    for edge in path.windows(2) {
        for e in &mut graph[edge[0]]{
            if e.0 == edge[1] {
                e.3 = true;
                break;
            }
        }
    }
}

fn unblock_network(graph: &mut Vec<Vec<(usize,u32,u32,bool)>>) {
    for adj in graph {
        for edge in adj {
            edge.3 = false;
        }
    }
}