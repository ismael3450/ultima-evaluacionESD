mod graph_core;
mod algorithms;
mod data_loader;
mod ui;

use graph_core::buscar_ciudad;

fn main() {
    // ── Bienvenida ────────────────────────────────────────────────────────────
    ui::mostrar_bienvenida();

    // ── Cargar la red (Steven - data_loader.rs) ───────────────────────────────
    let (grafo, origen) = data_loader::inicializar_red();
    ui::mostrar_resumen_grafo(&grafo);

    // ── Destino de prueba para la demo ────────────────────────────────────────
    let destino = buscar_ciudad(&grafo, "Ahuachapán")
        .expect("Ciudad destino no encontrada en la red");

    // ── BFS: orden de exploración desde San Salvador ──────────────────────────
    algorithms::bfs::mostrar_orden_visitas_bfs(&grafo, origen);

    // ── Comparativa BFS vs Dijkstra: San Salvador → Ahuachapán ───────────────
    let ruta_bfs = algorithms::bfs::ruta_mas_corta_bfs(&grafo, origen, destino);
    ui::mostrar_resultado_bfs(&grafo, origen, destino, ruta_bfs.as_ref());

    let ruta_dijkstra = algorithms::bfs::ruta_mas_corta_dijkstra(&grafo, origen, destino);
    ui::mostrar_resultado_dijkstra(&grafo, origen, destino, ruta_dijkstra.as_ref());

    // ── DFS: exploración y detección de ciclos ──────────────
    algorithms::dfs::ejecutar_dfs(&grafo, origen); // descomentar cuando Kevin termine

    // ── Cierre ────────────────────────────────────────────────────────────────
    ui::mostrar_reporte_final();
}
