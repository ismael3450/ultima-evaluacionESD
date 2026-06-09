// HOLA STEVEN:
// Te corresponde diseñar la inserción de los nodos y las aristas para poblar la red.
// Usa las funciones que Ismael preparó en `graph_core.rs`.

use crate::graph_core::{RedGrafo, crear_grafo_vacio, añadir_ciudad, conectar_ciudades, IdNodo};

/// Modela y puebla la red real simplificada (ej. Red de transporte entre ciudades).
/// Retorna el grafo completamente cargado y, si es necesario, el nodo inicial.
pub fn inicializar_red() -> (RedGrafo, IdNodo) {
    let mut grafo = crear_grafo_vacio();

    // 1. Añade las ciudades usando añadir_ciudad(&mut grafo, "Nombre")
    // 2. Conecta las ciudades con distancias usando conectar_ciudades(&mut grafo, desde, hasta, km)

    // Ejemplo temporal (reemplázalo por la red real que expongamos):
    let nodo_inicio = añadir_ciudad(&mut grafo, "San Salvador");
    let nodo_destino = añadir_ciudad(&mut grafo, "Santa Ana");
    conectar_ciudades(&mut grafo, nodo_inicio, nodo_destino, 65);

    (grafo, nodo_inicio)
}