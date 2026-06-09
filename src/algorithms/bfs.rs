use crate::graph_core::{self, IdNodo, RedGrafo};
use std::collections::{HashMap, HashSet, VecDeque};

/// Ejecuta BFS para encontrar el camino con el menor número de conexiones entre dos ciudades.
/// Retorna un Vector ordenado con la secuencia de nodos si el destino es alcanzable.
pub fn ruta_mas_corta_bfs(grafo: &RedGrafo, inicio: IdNodo, objetivo: IdNodo) -> Option<Vec<IdNodo>> {
    if inicio == objetivo {
        return Some(vec![inicio]);
    }

    let mut cola = VecDeque::new();
    let mut visitados = HashSet::new();
    let mut predecesores: HashMap<IdNodo, IdNodo> = HashMap::new();

    cola.push_back(inicio);
    visitados.insert(inicio);

    while let Some(actual) = cola.pop_front() {
        if actual == objetivo {
            return Some(reconstruir_ruta(inicio, objetivo, &predecesores));
        }

        // El recorrido de vecinos aprovecha la localidad de caché al acceder a arreglos compactos
        for vecino in grafo.neighbors(actual) {
            if visitados.insert(vecino) {
                predecesores.insert(vecino, actual);
                cola.push_back(vecino);
            }
        }
    }

    None
}

/// Reconstruye la ruta secuencial desde el objetivo hacia el inicio.
fn reconstruir_ruta(inicio: IdNodo, objetivo: IdNodo, predecesores: &HashMap<IdNodo, IdNodo>) -> Vec<IdNodo> {
    let mut ruta = Vec::new();
    let mut nodo_actual = objetivo;

    while let Some(&padre) = predecesores.get(&nodo_actual) {
        ruta.push(nodo_actual);
        nodo_actual = padre;
    }

    ruta.push(inicio);
    ruta.reverse();
    ruta
}

/// Recorrido auxiliar iterativo que simplemente imprime el orden de visitas del BFS estándar.
pub fn mostrar_orden_visitas_bfs(grafo: &RedGrafo, inicio: IdNodo) {
    let mut cola = VecDeque::new();
    let mut visitados = HashSet::new();

    cola.push_back(inicio);
    visitados.insert(inicio);

    println!("\n--- Órden de Exploración BFS ---");
    while let Some(actual) = cola.pop_front() {
        if let Some(nombre) = graph_core::obtener_nombre(grafo, actual) {
            println!("[Nodo Visitado]: {}", nombre);
        }

        for vecino in grafo.neighbors(actual) {
            if visitados.insert(vecino) {
                cola.push_back(vecino);
            }
        }
    }
    println!("--------------------------------\n");
}

// =========================================================================
// SECCIÓN DE PRUEBAS UNITARIAS
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph_core::{añadir_ciudad, conectar_ciudades, crear_grafo_vacio};

    #[test]
    fn test_ruta_mas_corta_bfs_valida() {
        let mut grafo = crear_grafo_vacio();
        let ss = añadir_ciudad(&mut grafo, "San Salvador");
        let sa = añadir_ciudad(&mut grafo, "Santa Ana");
        let sm = añadir_ciudad(&mut grafo, "San Miguel");
        let ah = añadir_ciudad(&mut grafo, "Ahuachapán");

        conectar_ciudades(&mut grafo, ss, sa, 65);
        conectar_ciudades(&mut grafo, sa, ah, 35);
        conectar_ciudades(&mut grafo, ss, sm, 138);

        let ruta_optima = ruta_mas_corta_bfs(&grafo, ss, ah);

        assert!(ruta_optima.is_some(), "El algoritmo no encontró una ruta que sí existe.");

        let ruta = ruta_optima.unwrap();
        assert_eq!(ruta, vec![ss, sa, ah], "El BFS calculó una ruta incorrecta.");
    }

    #[test]
    fn test_ruta_inexistente() {
        let mut grafo = crear_grafo_vacio();
        let ss = añadir_ciudad(&mut grafo, "San Salvador");
        let isla = añadir_ciudad(&mut grafo, "Isla Aislada");

        let ruta = ruta_mas_corta_bfs(&grafo, ss, isla);
        assert!(ruta.is_none(), "El algoritmo inventó una ruta a un nodo desconectado.");
    }
}