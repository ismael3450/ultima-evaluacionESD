mod graph_core;
mod algorithms;
mod data_loader;
mod ui;

use graph_core::{añadir_ciudad, conectar_ciudades, crear_grafo_vacio, obtener_nombre};

fn main() {
    ui::mostrar_bienvenida();

    // --- Construcción del grafo (temporal hasta que Steven termine data_loader) ---
    let mut grafo = crear_grafo_vacio();

    let ss = añadir_ciudad(&mut grafo, "San Salvador");
    let sa = añadir_ciudad(&mut grafo, "Santa Ana");
    let sm = añadir_ciudad(&mut grafo, "San Miguel");
    let ah = añadir_ciudad(&mut grafo, "Ahuachapán");
    let so = añadir_ciudad(&mut grafo, "Sonsonate");

    conectar_ciudades(&mut grafo, ss, sa, 65);
    conectar_ciudades(&mut grafo, sa, ah, 35);
    conectar_ciudades(&mut grafo, ss, sm, 138);
    conectar_ciudades(&mut grafo, ss, so, 74);
    conectar_ciudades(&mut grafo, so, sa, 40);

    // --- BFS: orden de exploración ---
    algorithms::bfs::mostrar_orden_visitas_bfs(&grafo, ss);

    // --- Comparativa BFS vs Dijkstra: SS → AH ---
    let origen = ss;
    let destino = ah;
    let nombre_origen = obtener_nombre(&grafo, origen).unwrap_or("?");
    let nombre_destino = obtener_nombre(&grafo, destino).unwrap_or("?");

    println!("Ruta: {} → {}\n", nombre_origen, nombre_destino);

    match algorithms::bfs::ruta_mas_corta_bfs(&grafo, origen, destino) {
        Some(ruta) => {
            let nombres: Vec<&str> = ruta.iter()
                .filter_map(|&id| obtener_nombre(&grafo, id))
                .collect();
            println!("  BFS  (menos saltos): {} ({} conexiones)", nombres.join(" → "), nombres.len() - 1);
        }
        None => println!("  BFS: no existe ruta."),
    }

    match algorithms::bfs::ruta_mas_corta_dijkstra(&grafo, origen, destino) {
        Some((km, ruta)) => {
            let nombres: Vec<&str> = ruta.iter()
                .filter_map(|&id| obtener_nombre(&grafo, id))
                .collect();
            println!("  Dijkstra (menos km): {} ({}km)", nombres.join(" → "), km);
        }
        None => println!("  Dijkstra: no existe ruta."),
    }

    println!();
    // algorithms::dfs::ejecutar_dfs(&grafo, ss); // Kevin
}