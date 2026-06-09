use crate::graph_core::{self, IdNodo, RedGrafo};
use petgraph::algo::dijkstra;
use std::collections::{HashMap, HashSet, VecDeque};

/// Ruta con menor número de saltos. Complejidad O(V + E).
pub fn ruta_mas_corta_bfs(grafo: &RedGrafo, inicio: IdNodo, objetivo: IdNodo) -> Option<Vec<IdNodo>> {
    if grafo.node_weight(inicio).is_none() || grafo.node_weight(objetivo).is_none() {
        return None;
    }
    if inicio == objetivo {
        return Some(vec![inicio]);
    }

    let mut cola: VecDeque<IdNodo> = VecDeque::new();
    let mut visitados: HashSet<IdNodo> = HashSet::new();
    let mut predecesores: HashMap<IdNodo, IdNodo> = HashMap::new();

    cola.push_back(inicio);
    visitados.insert(inicio);

    while let Some(actual) = cola.pop_front() {
        if actual == objetivo {
            return Some(reconstruir_ruta(inicio, objetivo, &predecesores));
        }
        for vecino in grafo.neighbors(actual) {
            if visitados.insert(vecino) {
                predecesores.insert(vecino, actual);
                cola.push_back(vecino);
            }
        }
    }

    None
}

/// Ruta con menor distancia total en km usando Dijkstra. Complejidad O((V + E) log V).
pub fn ruta_mas_corta_dijkstra(grafo: &RedGrafo, inicio: IdNodo, objetivo: IdNodo) -> Option<(u32, Vec<IdNodo>)> {
    if grafo.node_weight(inicio).is_none() || grafo.node_weight(objetivo).is_none() {
        return None;
    }

    let costos = dijkstra(grafo, inicio, Some(objetivo), |e| *e.weight());
    let costo_total = *costos.get(&objetivo)?;

    let mut ruta = vec![objetivo];
    let mut actual = objetivo;

    while actual != inicio {
        let costo_actual = *costos.get(&actual)?;
        let padre = grafo
            .neighbors(actual)
            .filter_map(|vecino| {
                let costo_vecino = costos.get(&vecino)?;
                let peso = grafo.find_edge(vecino, actual).and_then(|e| grafo.edge_weight(e)).copied()?;
                (costo_vecino + peso == costo_actual).then_some(vecino)
            })
            .next()?;

        ruta.push(padre);
        actual = padre;
    }

    ruta.reverse();
    Some((costo_total, ruta))
}

pub fn mostrar_orden_visitas_bfs(grafo: &RedGrafo, inicio: IdNodo) {
    if grafo.node_weight(inicio).is_none() {
        eprintln!("[BFS] Nodo de inicio inválido.");
        return;
    }

    let mut cola: VecDeque<IdNodo> = VecDeque::new();
    let mut visitados: HashSet<IdNodo> = HashSet::new();

    cola.push_back(inicio);
    visitados.insert(inicio);

    println!("\n--- Orden de Exploración BFS ---");
    while let Some(actual) = cola.pop_front() {
        if let Some(nombre) = graph_core::obtener_nombre(grafo, actual) {
            println!("  [Visitado]: {}", nombre);
        }
        for vecino in grafo.neighbors(actual) {
            if visitados.insert(vecino) {
                cola.push_back(vecino);
            }
        }
    }
    println!("--------------------------------\n");
}

fn reconstruir_ruta(inicio: IdNodo, objetivo: IdNodo, predecesores: &HashMap<IdNodo, IdNodo>) -> Vec<IdNodo> {
    let mut ruta = Vec::new();
    let mut actual = objetivo;

    while let Some(&padre) = predecesores.get(&actual) {
        ruta.push(actual);
        actual = padre;
    }

    ruta.push(inicio);
    ruta.reverse();
    ruta
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph_core::{añadir_ciudad, conectar_ciudades, crear_grafo_vacio};
    use petgraph::graph::NodeIndex;

    fn grafo_el_salvador() -> (RedGrafo, NodeIndex, NodeIndex, NodeIndex, NodeIndex) {
        let mut g = crear_grafo_vacio();
        let ss = añadir_ciudad(&mut g, "San Salvador");
        let sa = añadir_ciudad(&mut g, "Santa Ana");
        let sm = añadir_ciudad(&mut g, "San Miguel");
        let ah = añadir_ciudad(&mut g, "Ahuachapán");
        conectar_ciudades(&mut g, ss, sa, 65);
        conectar_ciudades(&mut g, sa, ah, 35);
        conectar_ciudades(&mut g, ss, sm, 138);
        (g, ss, sa, sm, ah)
    }

    #[test]
    fn test_bfs_ruta_valida() {
        let (g, ss, sa, _, ah) = grafo_el_salvador();
        assert_eq!(ruta_mas_corta_bfs(&g, ss, ah).unwrap(), vec![ss, sa, ah]);
    }

    #[test]
    fn test_bfs_mismo_origen_destino() {
        let (g, ss, _, _, _) = grafo_el_salvador();
        assert_eq!(ruta_mas_corta_bfs(&g, ss, ss).unwrap(), vec![ss]);
    }

    #[test]
    fn test_bfs_destino_inalcanzable() {
        let mut g = crear_grafo_vacio();
        let ss = añadir_ciudad(&mut g, "San Salvador");
        let isla = añadir_ciudad(&mut g, "Isla Aislada");
        assert!(ruta_mas_corta_bfs(&g, ss, isla).is_none());
    }

    #[test]
    fn test_bfs_nodo_invalido() {
        let (g, ss, _, _, _) = grafo_el_salvador();
        let id_falso = NodeIndex::new(999);
        assert!(ruta_mas_corta_bfs(&g, ss, id_falso).is_none());
        assert!(ruta_mas_corta_bfs(&g, id_falso, ss).is_none());
    }

    #[test]
    fn test_dijkstra_distancia_correcta() {
        let (g, ss, _, _, ah) = grafo_el_salvador();
        let (costo, _) = ruta_mas_corta_dijkstra(&g, ss, ah).unwrap();
        assert_eq!(costo, 100); // SS→SA(65) + SA→AH(35)
    }

    // BFS elige A→C (1 salto). Dijkstra elige A→B→C (2km < 9km).
    #[test]
    fn test_bfs_vs_dijkstra_difieren() {
        let mut g = crear_grafo_vacio();
        let a = añadir_ciudad(&mut g, "A");
        let b = añadir_ciudad(&mut g, "B");
        let c = añadir_ciudad(&mut g, "C");
        conectar_ciudades(&mut g, a, b, 1);
        conectar_ciudades(&mut g, b, c, 1);
        conectar_ciudades(&mut g, a, c, 9);

        assert_eq!(ruta_mas_corta_bfs(&g, a, c).unwrap(), vec![a, c]);
        let (costo, ruta) = ruta_mas_corta_dijkstra(&g, a, c).unwrap();
        assert_eq!(costo, 2);
        assert_eq!(ruta, vec![a, b, c]);
    }
}