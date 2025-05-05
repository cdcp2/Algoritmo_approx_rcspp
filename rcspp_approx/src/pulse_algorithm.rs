use std::{cmp::Reverse, collections::BinaryHeap};

#[derive(Debug, Clone)]
pub struct Pulse {
    pub path: Vec<usize>,
    pub cost: u32,
    pub consumption: u32,
    pub last_node: usize,
    pub visited: Vec<bool>,
}

impl Pulse {
    fn new(path: Vec<usize>, cost: u32, consumption: u32, last_node: usize, visited: Vec<bool>) -> Self {
        Pulse {path, cost, consumption, last_node, visited}
    }

    fn add_edge(&mut self, edge: (usize, u32, u32)) {
        self.path.push(edge.0);
        self.visited[edge.0] = true;
        self.last_node = edge.0;
        self.cost += edge.1;
        self.consumption += edge.2;
    }

    fn remove_edge(&mut self, edge: (usize, u32, u32)) {
        self.path.pop();
        self.visited[edge.0] = false;
        self.last_node = *self.path.last().unwrap();
        self.cost -= edge.1;
        self.consumption -= edge.2;
        
    }

    // reglas de poda
    fn is_dominated(&self, other: & [Option<(u32,u32)>; 3]) -> bool {
        let mut is_dom: u8 = 0;

        for i in 0..3 {
            match other[i] {
                Some((cost, consumption)) => if self.consumption >= consumption && self.cost >= cost {is_dom+=1},
                None => ()
            } 
        }

        is_dom > 2
    }

    fn check_bounds(&self, primal_bound: u32, minimum_cost: &Vec<u32>) -> bool {
        minimum_cost[self.last_node] < primal_bound && self.cost + minimum_cost[self.last_node] <= primal_bound
    }

    fn check_feasibility(&self, resource_limit: u32, minimum_consumption: &Vec<u32>) -> bool {
        minimum_consumption[self.last_node] <= resource_limit && self.consumption + minimum_consumption[self.last_node] <= resource_limit
    }

    // para actualizar los lables, esta es una manera, la otra es tener multiples labels por nodo
    fn replace_first(&self, other: Option<(u32,u32)>) -> bool {
        match other {
            Some((cost, _consumption)) => self.cost < cost,
            None => true
        }
    }

    fn replace_second(&self, other: Option<(u32,u32)>) -> bool {
        match other {
            Some((_cost, consumption)) => self.consumption < consumption,
            None => true
        }
    }

    fn replace_third(&self, other: Option<(u32,u32)>) -> bool {
        match other {
            Some((_cost, _consumption)) => rand::random_bool(0.5),
            None => true
        }
    }
}


pub fn pulse_algorithm(graph: &Vec<Vec<(usize, u32, u32)>>, s: usize, e:usize, resource_limit: u32)-> Option<Pulse> {
    //every edge is (node, cost, consumption)
    let mut visited = vec![false; graph.len()];
    visited[s] = true;
    let mut curr = Pulse::new(vec![s], 0, 0, s, visited);
    let mut labels: Vec<[Option<(u32, u32)>; 3]> = vec![[None,None,None]; graph.len()];

    let mut primal_bound = u32::MAX;
    let minimum_consumption = get_bounds(&graph, e, |(_a, _b,c)| c);
    //println!("Minimum consumption: {:?}", minimum_consumption);
    let minimum_cost = get_bounds(&graph, e, |(_a, b, _c)| b);
    //println!("Minimum cost: {:?}", minimum_cost);
    let mut best_path = None;
    
    expand_pulse(&graph, s, e, resource_limit, &mut primal_bound, &minimum_cost, &minimum_consumption, &mut labels, &mut curr, &mut best_path);

    best_path
}


fn expand_pulse(graph: &Vec<Vec<(usize, u32, u32)>>, 
            s: usize, 
            e: usize, 
            resource_limit: u32, 
            primal_bound: &mut u32, 
            minimum_cost: &Vec<u32>, 
            minimum_consumption: &Vec<u32>, 
            labels: &mut Vec<[Option<(u32,u32)>; 3]>,
            curr: &mut Pulse,
            best_path: &mut Option<Pulse>) {
    
    // uses backtracking with prunning strategies to find the best path
    

    //println!("lables: {:?}", labels);
    if curr.replace_first(labels[curr.last_node][0]) {
        labels[curr.last_node][0] = Some((curr.cost, curr.consumption))
    }

    if curr.replace_second(labels[curr.last_node][1]) {
        labels[curr.last_node][1] = Some((curr.cost, curr.consumption))
    }

    if curr.replace_third(labels[curr.last_node][2]) {
        labels[curr.last_node][2] = Some((curr.cost, curr.consumption))
    }
    //println!("curr: {:?}", curr);
    //println!("labels: {:?}", labels);
    for &edge in &graph[curr.last_node] {
        if curr.visited[edge.0] {
            continue;
        }

        curr.add_edge(edge);

        if curr.is_dominated(&labels[curr.last_node]) 
        || !curr.check_bounds(*primal_bound, minimum_cost) 
        || !curr.check_feasibility(resource_limit, minimum_consumption) {
            curr.remove_edge(edge);
            continue;
        }
            
        //if we get to the end, it updates the primal bound and the best path
        if curr.last_node == e  && curr.cost < *primal_bound{
            *primal_bound = curr.cost;
            *best_path = Some(curr.clone());
        } else if curr.last_node != e {
            //if we are not at the end, we call the function recursively
            expand_pulse(graph, s, e, resource_limit, primal_bound, minimum_cost, minimum_consumption, labels, curr, best_path);
        }
            
        curr.remove_edge(edge);
            
    }
    
}
    


fn get_bounds(graph: &Vec<Vec<(usize, u32, u32)>>, s: usize, cost: fn((usize, u32,u32))->u32)-> Vec<u32> {
    // Reverse graph for Dijkstra's algorithm so we can find the minimum cost to each node from the target node
    // so we also get the minimum cost from every node to the target node
    // with the function cost, it also give us a way to get the minimum consumption without reapeating code
    let graph_rev = graph.iter().enumerate().fold(vec![Vec::new(); graph.len()],
     |mut acc: Vec<Vec<(usize, u32)>>, (i, adj)| {
        adj.iter().for_each(|&edge| {
            acc[edge.0].push((i, cost(edge)));
            
        });

        acc
    });

    let mut heap: BinaryHeap<Reverse<(u32, usize)>> = BinaryHeap::new();
    heap.push(Reverse((0, s))); // (costo acumulado, consumo acumulado, nodo actual)
    let mut min_consumption = vec![u32::MAX; graph.len()];
    min_consumption[s] = 0;

    while let Some(Reverse((resource, node))) = heap.pop() {
        if resource > min_consumption[node] {
            continue;
        }

        for &(next_node, edge_resource) in &graph_rev[node] {

            let new_resource = resource + edge_resource;

            if new_resource < min_consumption[next_node] {
                min_consumption[next_node] = new_resource;
                heap.push(Reverse((new_resource, next_node)));
            }
        }
    }

    min_consumption
}
