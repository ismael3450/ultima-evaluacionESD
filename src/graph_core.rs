use petgraph::graph::{NodeIndex, UnGraph};

pub type Distancia = u32;
pub type RedGrafo = UnGraph<String, Distancia>;
pub type IdNodo = NodeIndex;

/// Inicializa un grafo vacío de tipo UnGraph (No dirigido y ponderado).
/// Almacena internamente los nodos de forma contigua en memoria, garantizando
/// una excelente localidad de caché.
pub fn crear_grafo_vacio() -> RedGrafo {
    RedGrafo::default()
}

/// Añade una ciudad al grafo y retorna su identificador único (NodeIndex).
/// Implementa una salvaguarda que busca si la ciudad ya existe para evitar la
/// duplicación de nodos en memoria durante la carga de datos.
pub fn añadir_ciudad(grafo: &mut RedGrafo, nombre: &str) -> IdNodo {
    if let Some(id_existente) = buscar_ciudad(grafo, nombre) {
        return id_existente;
    }
    grafo.add_node(nombre.to_string())
}

/// Conecta dos ciudades mediante una arista ponderada (distancia).
/// Valida que no exista una conexión previa para asegurar la integridad de la red.
pub fn conectar_ciudades(grafo: &mut RedGrafo, desde: IdNodo, hasta: IdNodo, distancia: Distancia) {
    if !grafo.contains_edge(desde, hasta) {
        grafo.add_edge(desde, hasta, distancia);
    }
}

// =========================================================================
// MÉTODOS DE CONSULTA DE ALTA EFICIENCIA (Para algoritmos y UI)
// =========================================================================

/// Obtiene de forma segura el nombre de una ciudad a partir de su IdNodo.
/// Esencial para que el BFS, DFS y la UI puedan imprimir texto en lugar de índices.
pub fn obtener_nombre(grafo: &RedGrafo, id: IdNodo) -> Option<&str> {
    grafo.node_weight(id).map(|s| s.as_str())
}

/// Busca una ciudad por su nombre exacto y devuelve su IdNodo.
/// Utiliza los iteradores nativos de petgraph optimizados para recorrer la memoria contigua.
pub fn buscar_ciudad(grafo: &RedGrafo, nombre: &str) -> Option<IdNodo> {
    grafo.node_indices().find(|&idx| {
        if let Some(n) = grafo.node_weight(idx) {
            n == nombre
        } else {
            false
        }
    })
}