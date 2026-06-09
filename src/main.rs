mod graph_core;
mod algorithms;
mod data_loader;
mod ui;

fn main() {
    // 1. Interfaz inicial (Trabajo de Steven)
    // ui::mostrar_bienvenida();
    println!("--- Sistema de Monitoreo de Red (Versión de Desarrollo) ---");

    // 2. Carga del grafo en memoria (Trabajo de Steven)
    // let (grafo, nodo_inicio) = data_loader::inicializar_red();

    // ==========================================
    // CÓDIGO TEMPORAL PARA QUE PRUEBES DE BFS:
    // (Bórralo cuando Steven termine data_loader)
    let mut grafo = graph_core::crear_grafo_vacio();
    let ss = graph_core::añadir_ciudad(&mut grafo, "San Salvador");
    let sa = graph_core::añadir_ciudad(&mut grafo, "Santa Ana");
    graph_core::conectar_ciudades(&mut grafo, ss, sa, 65);

    algorithms::bfs::mostrar_orden_visitas_bfs(&grafo, ss);
    // ==========================================

    // 3. Ejecución de algoritmos para comparativa
    // algorithms::dfs::ejecutar_dfs(&grafo, nodo_inicio); // Trabajo de Kevin

    // 4. Resultados finales
    // ui::mostrar_reporte_final();
}