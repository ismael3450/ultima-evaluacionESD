// HOLA KEVIN:
// Te corresponde implementar el algoritmo secundario: Recorrido en Profundidad (DFS).
// Recuerda que la guía pide usarlo para detectar si existen ciclos en la red.
// Utiliza los tipos definidos en `graph_core`.

use crate::graph_core::{RedGrafo, IdNodo};
use std::collections::HashSet;

/// Ejecuta un recorrido en profundidad (DFS) para detectar ciclos o explorar la red.
pub fn ejecutar_dfs(grafo: &RedGrafo, inicio: IdNodo) {
    println!("\n--- Iniciando Recorrido DFS (Trabajo de Kevin) ---");

    // Tu código del DFS va aquí...
    // Pista: Puedes usar recursividad o una pila (Stack) manual, rastreando los nodos visitados.

    println!("------------------------------------------------\n");
}

// 🧪 SECCIÓN DE PRUEBAS UNITARIAS
// La rúbrica evalúa la lógica del código. Aquí debes estructurar los tests
// para validar que las rutas calculadas por el programa sean correctas.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph_core::{crear_grafo_vacio, añadir_ciudad, conectar_ciudades};

    #[test]
    fn test_deteccion_de_ciclos() {
        // Levanta un grafo pequeño de prueba y asegura con un assert! que tu DFS funcione
    }
}