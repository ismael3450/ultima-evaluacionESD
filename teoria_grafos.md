# Investigación Teórica: Fundamentos de Grafos
## 1. Justificación Matemática de los Grafos

Un grafo $G$ se define formalmente como un par ordenado $G = (V, A)$, donde:
*   $V$ es un conjunto no vacío de **vértices** o nodos.
*   $A$ es un conjunto de **aristas** o arcos que conectan pares de elementos de $V$.

### Modelado de la Red en el Proyecto
Para nuestro proyecto, implementamos un grafo dirigido y ponderado utilizando la librería `petgraph`. La elección de este modelo matemático se justifica por la necesidad de representar relaciones asimétricas y costos de transición entre los nodos (por ejemplo, distancias, tiempos o pesos de red).

*   **Direccionalidad:** Si existe una arista $(u, v) \in A$, no necesariamente existe $(v, u) \in A$, lo que permite modelar flujos unidireccionales.
*   **Ponderación:** Cada arista tiene asociado un peso mediante una función de costo $w: A \rightarrow \mathbb{R}^+$, indispensable para que los algoritmos primario y secundario calculen las rutas óptimas basándose en la minimización de este valor.

---

## 2. Análisis de Complejidad Espacial

La eficiencia en el uso de la memoria depende críticamente de la estructura de datos elegida para representar el grafo en la computadora. Las dos representaciones más comunes son la **Matriz de Adyacencia** y la **Lista de Adyacencia**.

### Matriz de Adyacencia
Consiste en una matriz bidimensional de tamaño $|V| \times |V|$. 
*   **Complejidad Espacial:** $\mathcal{O}(|V|^2)$.
*   **Desventaja:** Si el grafo es **disperso** (tiene pocas aristas en comparación con el máximo posible), la matriz almacena una enorme cantidad de ceros o valores nulos, desperdiciando memoria de forma cuadrática.

### Lista de Adyacencia (Enfoque de `petgraph` en Rust)
`petgraph` utiliza internamente una variación optimizada de listas de adyacencia basada en vectores indexados (`Graph<NodeWeight, EdgeWeight>`). En lugar de usar punteros tradicionales dispersos por la memoria, almacena los nodos y las aristas en arreglos contiguos de memoria.
*   **Complejidad Espacial:** $\mathcal{O}(|V| + |A|)$.
*   **Ventaja en Grafos Dispersos:** El consumo de memoria es estrictamente proporcional al tamaño real de la red (nodos más las conexiones existentes), eliminando el desperdicio.

### Localidad de Caché y Gestión de Memoria en Rust
A diferencia de lenguajes con Recolector de Basura (como Java o Python) que dispersan los objetos en el *Heap* de forma fragmentada, la estructura de `petgraph` aprovecha el sistema de *Ownership* (propiedad) de Rust. Al empaquetar los datos en vectores contiguos:
1.  Se reduce drásticamente el *cache miss* (fallo de caché). La CPU puede precargar bloques enteros de nodos adyacentes a la memoria caché L1/L2/L3.
2.  La localidad espacial mejora el rendimiento de los algoritmos de recorrido, acelerando la velocidad de lectura en comparación con las listas enlazadas tradicionales basadas en punteros dispersos.

---

## 3. Comparativa de Complejidad de Algoritmos

A continuación, se presenta la base analítica para la comparación técnica entre el algoritmo primario y el algoritmo secundario implementados en el código.

| Métrica / Característica | Algoritmo Primario (Ej. Dijkstra / BFS) | Algoritmo Secundario (Ej. Bellman-Ford / DFS) |
| :--- | :--- | :--- |
| **Complejidad Temporal** | $\mathcal{O}((\|V\| + \|A\|) \log \|V\|)$ | $\mathcal{O}(\|V\| \cdot \|A\|)$ o $\mathcal{O}(\|V\| + \|A\|)$ |
| **Complejidad Espacial** | $\mathcal{O}(\|V\|)$ (Cola de prioridad/visita) | $\mathcal{O}(\|V\|)$ (Pila de llamadas / distancias) |
| **Manejo de Pesos** | Solo pesos positivos ($\ge 0$) | Puede detectar ciclos negativos (si aplica) |
