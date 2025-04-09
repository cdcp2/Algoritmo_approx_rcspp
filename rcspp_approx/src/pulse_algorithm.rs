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
        self.cost += edge.1;
        self.consumption += edge.2;
    }

    fn remove_edge(&mut self, edge: (usize, u32, u32)) {
        if let Some(pos) = self.path.iter().position(|&x| x == edge.0) {
            self.path.remove(pos);
            self.cost -= edge.1;
            self.consumption -= edge.2;
        }
    }

    fn check_dominance(&self, other: &Option<Self>) -> bool {
        match other {
            Some(other) => self.cost >= other.cost && self.consumption >= other.consumption,
            None => false,
        }
    }

    fn check_bounds(&self, primal_bound: u32, minimum_cost: &Vec<u32>) -> bool {
        self.cost + minimum_cost[self.last_node] <= primal_bound
    }

    fn check_feasibility(&self, resource_limit: u32, minimum_consumption: &Vec<u32>) -> bool {
        self.consumption + minimum_consumption[self.last_node] <= resource_limit
    }
}


pub fn pulse_algorithm(graph: Vec<Vec<(usize, u32, u32)>>, s: usize, e:usize, resource_limit: u32)-> Option<Pulse> {
    //every edge is (node, cost, consumption)
    let mut visited = vec![false; graph.len()];
    visited[s] = true;
    let mut curr = Pulse::new(vec![s], 0, 0, s, visited);
    let mut labels: Vec<Option<Pulse>> = vec![None; graph.len()];

    let mut primal_bound = u32::MAX;
    let minimum_consumption = get_bounds(&graph, e, |(_a, _b,c)| c);
    let minimum_cost = get_bounds(&graph, e, |(_a, b, _c)| b);
    let mut best_path = None;
    
    pulse(&graph, s, e, resource_limit, &mut primal_bound, &minimum_cost, &minimum_consumption, &mut labels, &mut curr, &mut best_path);

    best_path
}

fn pulse(graph: &Vec<Vec<(usize, u32, u32)>>, 
            s: usize, 
            e: usize, 
            resource_limit: u32, 
            primal_bound: &mut u32, 
            minimum_cost: &Vec<u32>, 
            minimum_consumption: &Vec<u32>, 
            labels: &mut Vec<Option<Pulse>>,
            curr: &mut Pulse,
            best_path: &mut Option<Pulse>) {

    if !curr.check_dominance(&labels[*curr.path.last().unwrap()]) {
        labels[curr.last_node] = Some(curr.clone());
        if curr.check_bounds(*primal_bound, minimum_cost) && curr.check_feasibility(resource_limit, minimum_consumption) {
            for (u, c, t) in &graph[curr.last_node] {
                if curr.visited[*u] {
                    continue;
                }

                curr.add_edge((*u, *c, *t));
                curr.visited[*u] = true;

                if curr.last_node == e  && curr.cost < *primal_bound{
                    *primal_bound = curr.cost;
                    *best_path = Some(curr.clone());
                } 
                pulse(graph, s, e, resource_limit, primal_bound, minimum_cost, minimum_consumption, labels, curr, best_path);
                curr.visited[*u] = false;
                curr.remove_edge((*u, *c, *t));
            }
        }
    }
    
}

fn get_bounds(graph: &Vec<Vec<(usize, u32, u32)>>, s: usize, cost: fn((usize, u32,u32))->u32)-> Vec<u32> {
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
