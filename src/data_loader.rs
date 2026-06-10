use crate::graph_core::{añadir_ciudad, conectar_ciudades, crear_grafo_vacio, IdNodo, RedGrafo};

/// Puebla el grafo con una red de transporte terrestre simplificada de El Salvador.
/// Retorna el grafo cargado y el nodo de San Salvador como punto de inicio.
pub fn inicializar_red() -> (RedGrafo, IdNodo) {
    let mut grafo = crear_grafo_vacio();

    // ── Nodos: ciudades principales de El Salvador ───────────────────────────
    let san_salvador = añadir_ciudad(&mut grafo, "San Salvador");
    let santa_ana    = añadir_ciudad(&mut grafo, "Santa Ana");
    let san_miguel   = añadir_ciudad(&mut grafo, "San Miguel");
    let sonsonate    = añadir_ciudad(&mut grafo, "Sonsonate");
    let la_libertad  = añadir_ciudad(&mut grafo, "La Libertad");
    let usulutan     = añadir_ciudad(&mut grafo, "Usulután");
    let san_vicente  = añadir_ciudad(&mut grafo, "San Vicente");
    let chalatenango = añadir_ciudad(&mut grafo, "Chalatenango");
    let cojutepeque  = añadir_ciudad(&mut grafo, "Cojutepeque");
    let zacatecoluca = añadir_ciudad(&mut grafo, "Zacatecoluca");
    let ahuachapan   = añadir_ciudad(&mut grafo, "Ahuachapán");

    // ── Aristas: distancias aproximadas en kilómetros ────────────────────────

    // Corredor central (Carretera Panamericana)
    conectar_ciudades(&mut grafo, san_salvador, cojutepeque,  34);
    conectar_ciudades(&mut grafo, cojutepeque,  san_vicente,  28);
    conectar_ciudades(&mut grafo, san_vicente,  usulutan,     57);
    conectar_ciudades(&mut grafo, usulutan,     san_miguel,   57);

    // Eje occidental
    conectar_ciudades(&mut grafo, san_salvador, santa_ana,    65);
    conectar_ciudades(&mut grafo, santa_ana,    ahuachapan,   35);
    conectar_ciudades(&mut grafo, santa_ana,    sonsonate,    40);

    // Corredor sur / costa
    conectar_ciudades(&mut grafo, san_salvador, la_libertad,  34);
    conectar_ciudades(&mut grafo, la_libertad,  sonsonate,    60);
    conectar_ciudades(&mut grafo, san_salvador, zacatecoluca, 56);
    conectar_ciudades(&mut grafo, zacatecoluca, usulutan,     35);

    // Conexiones norte
    conectar_ciudades(&mut grafo, san_salvador, chalatenango, 82);
    conectar_ciudades(&mut grafo, santa_ana,    chalatenango, 78);

    (grafo, san_salvador)
}
