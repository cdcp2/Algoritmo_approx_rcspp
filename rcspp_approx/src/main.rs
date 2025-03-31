use std::collections::{HashMap, BinaryHeap, HashSet};
use std::cmp::Ordering;
use rand::prelude::*;

// Estructura para representar el grafo
struct Graph {
    num_nodes: usize,
    edges: HashMap<usize, Vec<Edge>>,
}

struct Edge {
    to: usize,
    cost: i32,
    resources: Vec<i32>,
}

// Estructura para representar un cromosoma
#[derive(Clone)]
struct Chromosome {
    // Permutación de nodos intermedios (1 a n-2)
    genes: Vec<usize>,
    fitness: i32,
}

// Estado para el algoritmo A*
#[derive(Clone, PartialEq)]
struct State {
    node: usize,
    cost: i32,
    resources: Vec<i32>,
    priority: usize, // Basado en la permutación del cromosoma
}

impl Eq for State {}

// Implementamos Ord para que funcione con BinaryHeap
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Primero por prioridad, luego por costo
        other.priority.cmp(&self.priority)
            .then_with(|| other.cost.cmp(&self.cost))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Decodifica el cromosoma usando A* con prioridades basadas en la permutación
fn decode_chromosome(
    graph: &Graph, 
    chromosome: &Chromosome, 
    resource_limits: &[i32]
) -> Option<(Vec<usize>, i32)> {
    // Creamos un mapa de prioridades basado en la permutación
    let mut priorities = HashMap::new();
    for (i, &node) in chromosome.genes.iter().enumerate() {
        priorities.insert(node, i);
    }
    
    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::new();
    let mut came_from = HashMap::new();
    let mut cost_so_far = HashMap::new();
    
    // Nodo inicial (0)
    queue.push(State {
        node: 0,
        cost: 0,
        resources: vec![0; resource_limits.len()],
        priority: 0,
    });
    
    cost_so_far.insert(0, 0);
    
    while let Some(current) = queue.pop() {
        // Si llegamos al destino (n-1)
        if current.node == graph.num_nodes - 1 {
            // Reconstruir el camino
            let mut path = vec![current.node];
            let mut node = current.node;
            
            while let Some(&prev) = came_from.get(&node) {
                path.push(prev);
                node = prev;
            }
            
            path.reverse();
            return Some((path, current.cost));
        }
        
        // Si ya visitamos este nodo con estos recursos o mejores, continuamos
        if visited.contains(&current.node) {
            continue;
        }
        
        visited.insert(current.node);
        
        // Exploramos vecinos
        if let Some(edges) = graph.edges.get(&current.node) {
            for edge in edges {
                // Verificamos restricciones de recursos
                let mut new_resources = current.resources.clone();
                let mut valid = true;
                
                for i in 0..resource_limits.len() {
                    new_resources[i] += edge.resources[i];
                    if new_resources[i] > resource_limits[i] {
                        valid = false;
                        break;
                    }
                }
                
                if valid {
                    let new_cost = current.cost + edge.cost;
                    let new_priority = *priorities.get(&edge.to).unwrap_or(&usize::MAX);
                    
                    queue.push(State {
                        node: edge.to,
                        cost: new_cost,
                        resources: new_resources,
                        priority: new_priority,
                    });
                    
                    came_from.insert(edge.to, current.node);
                    cost_so_far.insert(edge.to, new_cost);
                }
            }
        }
    }
    
    None 
}

// Genera un cromosoma aleatorio (permutación de nodos intermedios)
fn generate_random_chromosome(num_nodes: usize, rng: &mut impl Rng) -> Chromosome {
    let mut genes: Vec<usize> = (1..num_nodes-1).collect();
    genes.shuffle(rng);
    
    Chromosome {
        genes,
        fitness: 0,
    }
}

// Cruce de orden (Order Crossover - OX)
fn crossover(parent1: &Chromosome, parent2: &Chromosome, rng: &mut impl Rng) -> Chromosome {
    let n = parent1.genes.len();
    let point1 = rng.random_range(0..n);
    let point2 = rng.random_range(0..n);
    
    let (start, end) = if point1 < point2 { (point1, point2) } else { (point2, point1) };
    
    // Inicializamos el hijo con marcadores
    let mut child_genes = vec![0; n];
    let mut used = vec![false; n + 2]; // +2 porque los genes van de 1 a n-2
    
    // Copiamos el segmento de parent1
    for i in start..=end {
        child_genes[i] = parent1.genes[i];
        used[parent1.genes[i]] = true;
    }
    
    // Rellenamos con elementos de parent2 en orden
    let mut j = (end + 1) % n;
    let mut parent2_idx = 0;
    
    while parent2_idx < n {
        let gene = parent2.genes[parent2_idx];
        if !used[gene] {
            child_genes[j] = gene;
            j = (j + 1) % n;
            if j == start {
                break;
            }
        }
        parent2_idx += 1;
    }
    
    Chromosome {
        genes: child_genes,
        fitness: 0,
    }
}

// Mutación (intercambio de dos posiciones aleatorias)
fn mutate(chromosome: &mut Chromosome, mutation_rate: f64, rng: &mut impl Rng) {
    if rng.random::<f64>() < mutation_rate {
        let n = chromosome.genes.len();
        let i = rng.random_range(0..n);
        let j = rng.random_range(0..n);
        chromosome.genes.swap(i, j);
    }
}

fn genetic_algorithm(
    graph: &Graph,
    population_size: usize,
    generations: usize,
    crossover_rate: f64,
    mutation_rate: f64,
    resource_limits: &[i32]
) -> Option<Vec<usize>> {
    let mut rng = rand::rng();
    
    // Generamos población inicial
    let mut population: Vec<Chromosome> = (0..population_size)
        .map(|_| generate_random_chromosome(graph.num_nodes, &mut rng))
        .collect();
    
    // Evaluamos fitness inicial
    for chromosome in &mut population {
        if let Some((_, cost)) = decode_chromosome(graph, chromosome, resource_limits) {
            chromosome.fitness = 10000 / (cost + 1);
        } else {
            chromosome.fitness = 0;
        }
    }
    
    for _ in 0..generations {
        // Ordenamos por fitness (descendente)
        population.sort_by(|a, b| b.fitness.cmp(&a.fitness));
        
        // Aplicamos elitismo (conservamos los mejores)
        let elite_size = (population_size as f64 * 0.1) as usize;
        let elite = population.iter().take(elite_size).cloned().collect::<Vec<_>>();
        
        let mut new_population = elite;
        
        // Generamos nueva población
        while new_population.len() < population_size {
            // Selección por torneo
            let parent1 = tournament_selection(&population, 3, &mut rng);
            let parent2 = tournament_selection(&population, 3, &mut rng);
            
            // Aplicamos cruce con cierta probabilidad
            let mut child = if rng.random::<f64>() < crossover_rate {
                crossover(&parent1, &parent2, &mut rng)
            } else if parent1.fitness > parent2.fitness {
                parent1.clone()
            } else {
                parent2.clone()
            };
            
            // Aplicamos mutación
            mutate(&mut child, mutation_rate, &mut rng);
            
            // Evaluamos fitness
            if let Some((_, cost)) = decode_chromosome(graph, &child, resource_limits) {
                child.fitness = 10000 / (cost + 1);
            } else {
                child.fitness = 0;
            }
            
            new_population.push(child);
        }
        
        population = new_population;
    }
    
    // Ordenamos población final
    population.sort_by(|a, b| b.fitness.cmp(&a.fitness));
    
    // Devolvemos el mejor camino
    decode_chromosome(graph, &population[0], resource_limits).map(|(path, _)| path)
}

// Selección por torneo
fn tournament_selection(
    population: &[Chromosome], 
    tournament_size: usize, 
    rng: &mut impl Rng
) -> Chromosome {
    let mut best = &population[rng.random_range(0..population.len())];
    
    for _ in 1..tournament_size {
        let competitor = &population[rng.random_range(0..population.len())];
        if competitor.fitness > best.fitness {
            best = competitor;
        }
    }
    
    (*best).clone()
}

fn main() {
    // Ejemplo de un grafo con 5 nodos (0 a 4)
    let mut graph = Graph {
        num_nodes: 5,
        edges: HashMap::new(),
    };
    
    // Definimos las aristas (origen, destino, costo, recursos)
    let edges = vec![
        (0, 1, 2, vec![1, 2]),
        (0, 2, 3, vec![2, 1]),
        (1, 2, 1, vec![1, 1]),
        (1, 3, 4, vec![2, 3]),
        (2, 3, 2, vec![1, 4]),
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
        Some(path) => println!("Mejor camino encontrado: {:?}", path),
        None => println!("No se encontró un camino válido con las restricciones dadas"),
    }
}
