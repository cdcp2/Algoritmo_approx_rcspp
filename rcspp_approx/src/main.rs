

mod bidirectional_pulse;
mod pulse_algorithm;
mod mult_obj_approach;
mod disjoint_path_approach;
mod edge_blocking_algo;
mod edge_penalization;

use std::{
    env,
    fs::File,
    io::{self, BufRead}, time::Instant,
};




fn main() -> io::Result<()> {
    // ── 1. Argumentos de línea de comandos ───────────────────────────────
    //    cargo run -- <archivo> <origen> <destino> <recurso_max>
    let args: Vec<String> = env::args().collect();
    if args.len() < 5 {
        eprintln!(
            "Uso: {} <archivo_entrada> <nodo_origen> <nodo_destino> <límite_recursos>",
            args[0]
        );
        std::process::exit(1);
    }

    let filename = &args[1];
    let s: usize = args[2].parse().expect("Nodo origen inválido");
    let e: usize = args[3].parse().expect("Nodo destino inválido");
    let resource_limit: u32 = args[4].parse().expect("Límite de recursos inválido");

    // ── 2. Leer todas las aristas y detectar el número de nodos ───────────
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    //  (u, v, costo, consumo)
    let mut edges: Vec<(usize, usize, u32, u32)> = Vec::new();
    let mut max_node = 0;

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        let u: usize = parts[0].parse().expect("Índice de nodo inválido");
        let v: usize = parts[1].parse().expect("Índice de nodo inválido");
        let cost: u32 = parts[2].parse().expect("Costo inválido");
        let cons: u32 = parts[3].parse().expect("Consumo inválido");

        edges.push((u, v, cost, cons));
        max_node = max_node.max(u).max(v);
    }

    // ── 3. Construir la lista de adyacencia ───────────────────────────────
    let mut graph: Vec<Vec<(usize, u32, u32)>> = vec![Vec::new(); max_node + 1];
    for (u, v, cost, cons) in edges {
        graph[u].push((v, cost, cons)); // Arista dirigida u → v
    }

    if s >= graph.len() || e >= graph.len() {
        eprintln!(
            "El nodo origen o destino está fuera de rango (0..{}).",
            graph.len() - 1
        );
        std::process::exit(1);
    }

    let mut pulse_cost = f64::MAX;
    let mut curr_cost=f64::MAX;
    println!("Corriendo Algoritmo del Pulso");
    let start = Instant::now();
    // ── 4. Lanzar el algoritmo Pulse ──────────────────────────────────────
    match pulse_algorithm::pulse_algorithm(&graph, s, e, resource_limit) {
        Some(best) => {println!(
            "Mejor camino: {:?}\nCosto total: {}\nConsumo total: {}",
            best.path, best.cost, best.consumption
        ); pulse_cost = best.cost as f64;},
        None => println!("No existe un camino factible con el límite de recursos dado."),
    }
    let duration = start.elapsed();
    println!("Duration: {:?}",duration);
    println!();

    println!("Corriendo Algoritmo de buscar en la frontera de pareto");
    let start = Instant::now();
    // ── 4. Lanzar el algoritmo Pulse ──────────────────────────────────────
    match mult_obj_approach::mult_obj(&graph, s, e, resource_limit, 0.1) {
        Some(best) => {println!(
            "Mejor camino: {:?}\nCosto total: {}\nConsumo total: {}",
            best.0, best.1, best.2
        ); curr_cost = best.1 as f64;},
        None => println!("No existe un camino factible con el límite de recursos dado."),
    }
    let duration = start.elapsed();
    println!("Duration: {:?}",duration);
    println!("Approximation: {}", curr_cost/pulse_cost);
    println!();

    println!("Corriendo Algoritmo de los caminos disyuntos");
    let start = Instant::now();
    // ── 4. Lanzar el algoritmo Pulse ──────────────────────────────────────
    match disjoint_path_approach::disjoint_algo(&graph, s, e, resource_limit){
        Some(best) => {println!(
            "Mejor camino: {:?}\nCosto total: {}\nConsumo total: {}",
            best.0, best.1, best.2
        ); curr_cost = best.1 as f64;},
        None => println!("No existe un camino factible con el límite de recursos dado."),
    }
    let duration = start.elapsed();
    println!("Duration: {:?}",duration);
    println!("Approximation: {}", curr_cost/pulse_cost);
    println!();

    println!("Corriendo edge block");
    let start = Instant::now();
    // ── 4. Lanzar el algoritmo Pulse ──────────────────────────────────────
    match edge_blocking_algo::edge_block(&graph, s, e, resource_limit){
        Some(best) => {println!(
            "Mejor camino: {:?}\nCosto total: {}\nConsumo total: {}",
            best.0, best.1, best.2
        ); curr_cost = best.1 as f64;},
        None => println!("No existe un camino factible con el límite de recursos dado."),
    }
    let duration = start.elapsed();
    println!("Duration: {:?}",duration);
    println!("Approximation: {}", curr_cost/pulse_cost);
    println!();

    println!("Corriendo edge penalization");
    let start = Instant::now();
    // ── 4. Lanzar el algoritmo Pulse ──────────────────────────────────────
    match edge_penalization::edge_penalization(&graph, s, e, resource_limit){
        Some(best) => {println!(
            "Mejor camino: {:?}\nCosto total: {}\nConsumo total: {}",
            best.0, best.1, best.2
        ); curr_cost = best.1 as f64;},
        None => println!("No existe un camino factible con el límite de recursos dado."),
    }
    let duration = start.elapsed();
    println!("Duration: {:?}",duration);
    println!("Approximation: {}", curr_cost/pulse_cost);
    println!();


    Ok(())
}