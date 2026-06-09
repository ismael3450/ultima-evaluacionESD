use petgraph::graph::{EdgeIndex, NodeIndex, UnGraph};

pub type Distancia = u32;
pub type RedGrafo = UnGraph<String, Distancia>;
pub type IdNodo = NodeIndex;
pub type IdArista = EdgeIndex;

pub fn crear_grafo_vacio() -> RedGrafo {
    RedGrafo::default()
}

/// Busca duplicados antes de insertar. Complejidad O(V).
pub fn añadir_ciudad(grafo: &mut RedGrafo, nombre: &str) -> IdNodo {
    if let Some(id) = buscar_ciudad(grafo, nombre) {
        return id;
    }
    grafo.add_node(nombre.to_string())
}

/// Retorna `false` si los nodos no existen o la arista ya existe.
pub fn conectar_ciudades(grafo: &mut RedGrafo, desde: IdNodo, hasta: IdNodo, distancia: Distancia) -> bool {
    if !grafo.node_indices().any(|n| n == desde) || !grafo.node_indices().any(|n| n == hasta) {
        return false;
    }
    if grafo.contains_edge(desde, hasta) {
        return false;
    }
    grafo.add_edge(desde, hasta, distancia);
    true
}

pub fn obtener_nombre(grafo: &RedGrafo, id: IdNodo) -> Option<&str> {
    grafo.node_weight(id).map(|s| s.as_str())
}

pub fn buscar_ciudad(grafo: &RedGrafo, nombre: &str) -> Option<IdNodo> {
    grafo.node_indices().find(|&idx| grafo.node_weight(idx).map_or(false, |n| n == nombre))
}

pub fn obtener_distancia(grafo: &RedGrafo, desde: IdNodo, hasta: IdNodo) -> Option<Distancia> {
    grafo.find_edge(desde, hasta).and_then(|a| grafo.edge_weight(a)).copied()
}

pub fn total_ciudades(grafo: &RedGrafo) -> usize { grafo.node_count() }
pub fn total_conexiones(grafo: &RedGrafo) -> usize { grafo.edge_count() }

#[cfg(test)]
mod tests {
    use super::*;

    fn grafo_de_prueba() -> (RedGrafo, IdNodo, IdNodo, IdNodo) {
        let mut g = crear_grafo_vacio();
        let ss = añadir_ciudad(&mut g, "San Salvador");
        let sa = añadir_ciudad(&mut g, "Santa Ana");
        let sm = añadir_ciudad(&mut g, "San Miguel");
        conectar_ciudades(&mut g, ss, sa, 65);
        conectar_ciudades(&mut g, ss, sm, 138);
        (g, ss, sa, sm)
    }

    #[test]
    fn test_no_duplica_ciudades() {
        let mut g = crear_grafo_vacio();
        let id1 = añadir_ciudad(&mut g, "San Salvador");
        let id2 = añadir_ciudad(&mut g, "San Salvador");
        assert_eq!(id1, id2);
        assert_eq!(total_ciudades(&g), 1);
    }

    #[test]
    fn test_no_duplica_aristas() {
        let (mut g, ss, sa, _) = grafo_de_prueba();
        assert!(!conectar_ciudades(&mut g, ss, sa, 99));
        assert_eq!(total_conexiones(&g), 2);
    }

    #[test]
    fn test_obtener_distancia() {
        let (g, ss, sa, _) = grafo_de_prueba();
        assert_eq!(obtener_distancia(&g, ss, sa), Some(65));
        assert_eq!(obtener_distancia(&g, sa, ss), Some(65));
    }

    #[test]
    fn test_buscar_ciudad() {
        let (g, ss, _, _) = grafo_de_prueba();
        assert_eq!(buscar_ciudad(&g, "San Salvador"), Some(ss));
        assert!(buscar_ciudad(&g, "Atlantis").is_none());
    }

    #[test]
    fn test_conectar_nodo_invalido() {
        let mut g = crear_grafo_vacio();
        let ss = añadir_ciudad(&mut g, "San Salvador");
        assert!(!conectar_ciudades(&mut g, ss, NodeIndex::new(999), 10));
    }
}