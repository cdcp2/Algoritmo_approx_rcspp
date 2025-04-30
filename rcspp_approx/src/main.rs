

mod bidirectional_pulse;
mod genetic_rcsp;
mod pulse_algorithm;
mod mult_obj_approach;
mod disjoint_path_approach;

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
    let start = Instant::now();
    // ── 4. Lanzar el algoritmo Pulse ──────────────────────────────────────
    match pulse_algorithm::pulse_algorithm(&graph, s, e, resource_limit) {
        Some(best) => println!(
            "Mejor camino: {:?}\nCosto total: {}\nConsumo total: {}",
            best.path, best.cost, best.consumption
        ),
        None => println!("No existe un camino factible con el límite de recursos dado."),
    }
    let duration = start.elapsed();
    println!("Duration: {:?}",duration);


    let start = Instant::now();
    // ── 4. Lanzar el algoritmo Pulse ──────────────────────────────────────
    match mult_obj_approach::mult_obj(&graph, s, e, resource_limit, 0.1) {
        Some(best) => println!(
            "Mejor camino: {:?}\nCosto total: {}\nConsumo total: {}",
            best.0, best.1, best.2
        ),
        None => println!("No existe un camino factible con el límite de recursos dado."),
    }
    let duration = start.elapsed();
    println!("Duration: {:?}",duration);

    let start = Instant::now();
    // ── 4. Lanzar el algoritmo Pulse ──────────────────────────────────────
    match disjoint_path_approach::disjoint_algo(&graph, s, e, resource_limit){
        Some(best) => println!(
            "Mejor camino: {:?}\nCosto total: {}\nConsumo total: {}",
            best.0, best.1, best.2
        ),
        None => println!("No existe un camino factible con el límite de recursos dado."),
    }
    let duration = start.elapsed();
    println!("Duration: {:?}",duration);

    Ok(())
}

fn test_mult_obj_algorithm() {
    println!("\n--- PRUEBA ALGORITMO MULTI-OBJETIVO ---");
    
    // Usar el mismo grafo de prueba que usaste para pulse_algorithm
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
    let source = 0;
    let target = 5;
    
    // Probar con diferentes límites de recursos
    let resource_limits = vec![2, 4, 6, 8, 10];
    
    for &limit in &resource_limits {
        println!("\nPrueba con límite de recursos = {}", limit);
        
        // Ejecutamos el algoritmo multi-objetivo con un incremento de 0.1 para lambda
        let result = mult_obj_approach::mult_obj(&graph, source, target, limit, 0.1);
        
        match result {
            Some((path, cost, resource)) => {
                println!("✅ Camino encontrado: {:?}", path);
                println!("   Costo total: {}", cost);
                println!("   Consumo de recursos: {}", resource);
            },
            None => {
                println!("❌ No se encontró un camino válido con el límite de recursos dado");
            }
        }
    }
}