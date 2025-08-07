use optimizador_de_rutas::*;
use std::collections::HashMap;
use petgraph::graph::UnGraph;

#[tokio::test]
async fn test_full_route_calculation() {
    // Coordenadas de prueba en Miraflores y Barranco, Lima.
    let start_lat = -12.115;
    let start_lon = -77.027;
    let end_lat = -12.148;
    let end_lon = -77.018;

    // Llamamos a la función principal con coordenadas de prueba
    // La función principal necesita ser modificada para devolver un valor o ser probada de una manera controlada.
    // Para simplificar, probaremos los componentes individuales que la componen.

    // 1. Verificar que el fetch de datos funcione y no retorne 0 elementos
    let res = fetch_overpass_data(start_lat, start_lon, end_lat, end_lon).await.unwrap();
    assert!(!res.elements.is_empty(), "La API de Overpass no devolvió datos para el área de prueba.");

    // 2. Construir el grafo y verificar que se creen nodos y aristas
    let (nodes, graph, node_indices) = build_graph_from_osm_data(res.elements).unwrap();
    assert!(!nodes.is_empty(), "No se crearon nodos a partir de los datos de OSM.");
    assert!(!graph.edge_count() > 0, "No se crearon aristas en el grafo.");
    
    // 3. Verificar que se encuentre una ruta
    let start_node_id = find_nearest_node(&nodes, start_lat, start_lon);
    let end_node_id = find_nearest_node(&nodes, end_lat, end_lon);

    let start_idx = *node_indices.get(&start_node_id).unwrap();
    let end_idx = *node_indices.get(&end_node_id).unwrap();
    
    let path = dijkstra_path(&graph, start_idx, end_idx).await;
    assert!(path.is_some(), "No se pudo encontrar una ruta entre los puntos de prueba.");
}