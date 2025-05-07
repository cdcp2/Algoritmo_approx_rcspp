use std::{
    env,
    fs::File,
    io::{self, BufRead},
    sync::{Arc, mpsc},
    thread,
    time::{Duration, Instant},
};

mod bidirectional_pulse;
mod pulse_algorithm;
mod mult_obj_approach;
mod disjoint_path_approach;
mod edge_blocking_algo;
mod edge_penalization;

fn main() -> io::Result<()> {
    // ── 1. Argumentos de línea de comandos ───────────────────────────────
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

    // ── 3. Construir la lista de adyacencia y compartir con Arc ───────────
    let mut graph: Vec<Vec<(usize, u32, u32)>> = vec![Vec::new(); max_node + 1];
    for (u, v, cost, cons) in edges {
        graph[u].push((v, cost, cons));
    }
    let graph = Arc::new(graph);

    if s >= graph.len() || e >= graph.len() {
        eprintln!(
            "El nodo origen o destino está fuera de rango (0..{}).",
            graph.len() - 1
        );
        std::process::exit(1);
    }

    let mut pulse_cost = f64::MAX;
    let mut curr_cost = f64::MAX;

    // ── 4. Ejecutar Pulse con timeout ────────────────────────────────────
    println!("Corriendo Algoritmo del Pulso (máximo 1 minutos)");
    let (tx, rx) = mpsc::channel();
    let graph_clone = Arc::clone(&graph);
    thread::spawn(move || {
        let result = pulse_algorithm::pulse_algorithm(&*graph_clone, s, e, resource_limit);
        let _ = tx.send(result);
    });

    match rx.recv_timeout(Duration::from_secs(60)) {
        Ok(Some(best)) => {
            println!(
                "Mejor camino: {:?}\nCosto total: {}\nConsumo total: {}",
                best.path, best.cost, best.consumption
            );
            pulse_cost = best.cost as f64;
        }
        Ok(None) => println!("No existe un camino factible con el límite de recursos dado."),
        Err(mpsc::RecvTimeoutError::Timeout) =>
            println!("Timeout: Pulse superó los 3 minutos. Pasando al siguiente algoritmo."),
        Err(e) => println!("Error recibiendo resultado de Pulse: {:?}", e),
    }

    println!();

    // ── 5. Resto de algoritmos ──────────────────────────────────────────
    println!("Corriendo Algoritmo de buscar en la frontera de pareto");
    let start = Instant::now();
    if let Some(best) = mult_obj_approach::mult_obj(&*graph, s, e, resource_limit, 0.1) {
        println!(
            "Mejor camino: {:?}\nCosto total: {}\nConsumo total: {}",
            best.0, best.1, best.2
        );
        curr_cost = best.1 as f64;
    } else {
        println!("No existe un camino factible con el límite de recursos dado.");
    }
    println!("Duración: {:?}\nApproximation: {}", start.elapsed(), curr_cost / pulse_cost);
    println!();

    println!("Corriendo Algoritmo de los caminos disyuntos");
    let start = Instant::now();
    if let Some(best) = disjoint_path_approach::disjoint_algo(&*graph, s, e, resource_limit) {
        println!(
            "Mejor camino: {:?}\nCosto total: {}\nConsumo total: {}",
            best.0, best.1, best.2
        );
        curr_cost = best.1 as f64;
    } else {
        println!("No existe un camino factible con el límite de recursos dado.");
    }
    println!("Duración: {:?}\nApproximation: {}", start.elapsed(), curr_cost / pulse_cost);
    println!();

    println!("Corriendo edge block");
    let start = Instant::now();
    if let Some(best) = edge_blocking_algo::edge_block(&*graph, s, e, resource_limit) {
        println!(
            "Mejor camino: {:?}\nCosto total: {}\nConsumo total: {}",
            best.0, best.1, best.2
        );
        curr_cost = best.1 as f64;
    } else {
        println!("No existe un camino factible con el límite de recursos dado.");
    }
    println!("Duración: {:?}\nApproximation: {}", start.elapsed(), curr_cost / pulse_cost);
    println!();

    println!("Corriendo edge penalization");
    let start = Instant::now();
    if let Some(best) = edge_penalization::edge_penalization(&*graph, s, e, resource_limit) {
        println!(
            "Mejor camino: {:?}\nCosto total: {}\nConsumo total: {}",
            best.0, best.1, best.2
        );
        curr_cost = best.1 as f64;
    } else {
        println!("No existe un camino factible con el límite de recursos dado.");
    }
    println!("Duración: {:?}\nApproximation: {}", start.elapsed(), curr_cost / pulse_cost);
    println!();

    Ok(())
}