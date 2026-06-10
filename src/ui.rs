use crate::graph_core::{obtener_distancia, obtener_nombre, total_ciudades, total_conexiones, IdNodo, RedGrafo};

const DOBLE: &str  = "════════════════════════════════════════════════════════";
const SIMPLE: &str = "────────────────────────────────────────────────────────";

// ── Pantalla de bienvenida ────────────────────────────────────────────────────

pub fn mostrar_bienvenida() {
    println!();
    println!("{DOBLE}");
    println!("    RED DE TRANSPORTE — EL SALVADOR  |  Grafos en Rust");
    println!("{DOBLE}");
    println!("  Estructura : Grafo no dirigido y ponderado (petgraph)");
    println!("  Algoritmos : BFS  — ruta con menos conexiones");
    println!("               Dijkstra — ruta con menor distancia en km");
    println!("               DFS  — exploración y detección de ciclos");
    println!("{DOBLE}");
    println!();
}

// ── Resumen del grafo cargado ─────────────────────────────────────────────────

pub fn mostrar_resumen_grafo(grafo: &RedGrafo) {
    println!("  Ciudades cargadas  : {}", total_ciudades(grafo));
    println!("  Conexiones activas : {}", total_conexiones(grafo));
    println!("{SIMPLE}");
    println!();
}

// ── Resultado de BFS ─────────────────────────────────────────────────────────

/// Muestra la ruta con menor número de saltos encontrada por BFS.
/// Recibe la ruta como Option<Vec<IdNodo>> directamente desde bfs::ruta_mas_corta_bfs.
pub fn mostrar_resultado_bfs(
    grafo: &RedGrafo,
    origen: IdNodo,
    destino: IdNodo,
    ruta: Option<&Vec<IdNodo>>,
) {
    let orig = obtener_nombre(grafo, origen).unwrap_or("?");
    let dest = obtener_nombre(grafo, destino).unwrap_or("?");

    println!("{DOBLE}");
    println!("  BFS — Ruta con menos conexiones");
    println!("  Origen  : {orig}");
    println!("  Destino : {dest}");
    println!("{SIMPLE}");

    match ruta {
        None => println!("  ✗  No existe ruta entre {orig} y {dest}."),
        Some(nodos) => {
            let mut km_total: u32 = 0;
            for par in nodos.windows(2) {
                let (a, b) = (par[0], par[1]);
                let km = obtener_distancia(grafo, a, b).unwrap_or(0);
                km_total += km;
                let na = obtener_nombre(grafo, a).unwrap_or("?");
                let nb = obtener_nombre(grafo, b).unwrap_or("?");
                println!("    {na}  ──[{km} km]──▶  {nb}");
            }
            let nombres: Vec<&str> = nodos.iter()
                .map(|&n| obtener_nombre(grafo, n).unwrap_or("?"))
                .collect();
            println!("{SIMPLE}");
            println!("  ✔  Ruta completa   : {}", nombres.join(" → "));
            println!("  ✔  Saltos          : {}", nodos.len().saturating_sub(1));
            println!("  ✔  Distancia total : {km_total} km");
        }
    }

    println!("{DOBLE}");
    println!();
}

// ── Resultado de Dijkstra ─────────────────────────────────────────────────────

/// Muestra la ruta con menor distancia en km encontrada por Dijkstra.
/// Recibe el resultado de bfs::ruta_mas_corta_dijkstra directamente.
pub fn mostrar_resultado_dijkstra(
    grafo: &RedGrafo,
    origen: IdNodo,
    destino: IdNodo,
    resultado: Option<&(u32, Vec<IdNodo>)>,
) {
    let orig = obtener_nombre(grafo, origen).unwrap_or("?");
    let dest = obtener_nombre(grafo, destino).unwrap_or("?");

    println!("{DOBLE}");
    println!("  DIJKSTRA — Ruta con menor distancia en km");
    println!("  Origen  : {orig}");
    println!("  Destino : {dest}");
    println!("{SIMPLE}");

    match resultado {
        None => println!("  ✗  No existe ruta entre {orig} y {dest}."),
        Some((costo_total, nodos)) => {
            for par in nodos.windows(2) {
                let (a, b) = (par[0], par[1]);
                let km = obtener_distancia(grafo, a, b).unwrap_or(0);
                let na = obtener_nombre(grafo, a).unwrap_or("?");
                let nb = obtener_nombre(grafo, b).unwrap_or("?");
                println!("    {na}  ──[{km} km]──▶  {nb}");
            }
            let nombres: Vec<&str> = nodos.iter()
                .map(|&n| obtener_nombre(grafo, n).unwrap_or("?"))
                .collect();
            println!("{SIMPLE}");
            println!("  ✔  Ruta completa   : {}", nombres.join(" → "));
            println!("  ✔  Saltos          : {}", nodos.len().saturating_sub(1));
            println!("  ✔  Distancia total : {costo_total} km");
        }
    }

    println!("{DOBLE}");
    println!();
}

// ── Resultado de DFS ─────────────────────────────────────────────────────────

/// Muestra el orden de visita DFS y si se detectó un ciclo.
/// orden_visita: secuencia de nodos visitados por DFS.
/// tiene_ciclo: true si DFS encontró un ciclo en la red.
pub fn mostrar_resultado_dfs(
    grafo: &RedGrafo,
    origen: IdNodo,
    orden_visita: &[IdNodo],
    tiene_ciclo: bool,
) {
    let orig = obtener_nombre(grafo, origen).unwrap_or("?");

    println!("{DOBLE}");
    println!("  DFS — Exploración en profundidad");
    println!("  Origen : {orig}");
    println!("{SIMPLE}");

    if orden_visita.is_empty() {
        println!("  (Sin nodos visitados)");
    } else {
        println!("  Orden de visita:");
        for (i, &nodo) in orden_visita.iter().enumerate() {
            let nombre = obtener_nombre(grafo, nodo).unwrap_or("?");
            println!("    {:>2}. {nombre}", i + 1);
        }
        println!();
        println!("  ✔  Nodos explorados : {}", orden_visita.len());
    }

    println!("{SIMPLE}");
    if tiene_ciclo {
        println!("  ⚠  Ciclo detectado en la red.");
    } else {
        println!("  ✔  No se detectaron ciclos.");
    }
    println!("{DOBLE}");
    println!();
}

// ── Reporte final ─────────────────────────────────────────────────────────────

pub fn mostrar_reporte_final() {
    println!("{DOBLE}");
    println!("  Ejecución finalizada.");
    println!("  red_ciudades_grafos  |  Estructuras de Datos — UES 2025");
    println!("{DOBLE}");
    println!();
}
