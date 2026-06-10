use crate::graph_core::{RedGrafo, IdNodo, obtener_nombre};
use std::collections::HashSet;

/// Ejecuta un recorrido en profundidad (DFS) para detectar ciclos y explorar la red.
/// Retorna `true` si encuentra algún ciclo (bucle circular) en la red de transporte.
pub fn ejecutar_dfs(grafo: &RedGrafo, inicio: IdNodo) -> bool {
    println!("\n--- Iniciando Recorrido DFS---");

    let mut visitados = HashSet::new();
    
    // Llamamos a la función auxiliar que rastrea al "nodo padre" para no confundir 
    // el camino de regreso en un grafo no dirigido con un ciclo real.
    let tiene_ciclo = detectar_ciclos_recursivo(grafo, inicio, None, &mut visitados);

    if tiene_ciclo {
        println!("\n[ALERTA] ¡Se ha detectado un ciclo/bucle cerrado en la red vial!");
    } else {
        println!("\nRed analizada por DFS con éxito. No se encontraron bucles.");
    }

    println!("------------------------------------------------\n");
    tiene_ciclo
}

/// Función auxiliar recursiva para DFS que realiza el rastreo de ciclos.
fn detectar_ciclos_recursivo(
    grafo: &RedGrafo,
    nodo_actual: IdNodo,
    padre: Option<IdNodo>,
    visitados: &mut HashSet<IdNodo>,
) -> bool {
    // Marcamos como visitado e imprimimos el progreso para la demostración en vivo
    visitados.insert(nodo_actual);
    let nombre = obtener_nombre(grafo, nodo_actual).unwrap_or("Desconocido");
    println!("-> Visitando: {}", nombre);

    // Explorar los vecinos usando las herramientas de petgraph
    for vecino in grafo.neighbors(nodo_actual) {
        // Al ser no dirigido, si el vecino es el nodo de donde venimos directo, lo ignoramos
        if Some(vecino) == padre {
            continue;
        }

        // Si el vecino ya fue visitado y no es el padre, ¡encontramos un ciclo!
        if visitados.contains(&vecino) {
            return true;
        }

        // Si no ha sido visitado, avanzamos en profundidad recursivamente
        if detectar_ciclos_recursivo(grafo, vecino, Some(nodo_actual), visitados) {
            return true;
        }
    }

    false
}

// para validar que las rutas calculadas por el programa sean correctas.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph_core::{crear_grafo_vacio, añadir_ciudad, conectar_ciudades};

    /// Configura una red de prueba lineal (sin ciclos) para validar caminos limpios.
    fn red_lineal_prueba() -> (RedGrafo, IdNodo) {
        let mut g = crear_grafo_vacio();
        let ss = añadir_ciudad(&mut g, "San Salvador");
        let sa = añadir_ciudad(&mut g, "Santa Ana");
        let ah = añadir_ciudad(&mut g, "Ahuachapán");

        // Ruta: San Salvador <-> Santa Ana <-> Ahuachapán
        conectar_ciudades(&mut g, ss, sa, 65);
        conectar_ciudades(&mut g, sa, ah, 35);

        (g, ss)
    }

    #[test]
    fn test_red_sin_ciclos() {
        let (g, inicio) = red_lineal_prueba();
        // El DFS debe retornar 'false' porque es una ruta abierta sin retornos cerrados
        assert!(!ejecutar_dfs(&g, inicio), "Error: Se reportó un ciclo donde no existe.");
    }

    #[test]
    fn test_deteccion_de_ciclos() {
        let (mut g, inicio) = red_lineal_prueba();
        
        // Obtenemos los IDs para cerrar un triángulo vial (Ciclo)
        let ss = crate::graph_core::buscar_ciudad(&g, "San Salvador").unwrap();
        let ah = crate::graph_core::buscar_ciudad(&g, "Ahuachapán").unwrap();

        // Forzamos un ciclo cerrando la ruta directo desde Ahuachapán de regreso a San Salvador
        // Ciclo creado: San Salvador <-> Santa Ana <-> Ahuachapán <-> San Salvador
        conectar_ciudades(&mut g, ah, ss, 100);

        // El assert! debe verificar que devuelva 'true'
        assert!(ejecutar_dfs(&g, inicio), "Error: El DFS no detectó el bucle cerrado en la red.");
    }
}