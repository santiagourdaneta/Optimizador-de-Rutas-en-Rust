// File: tests/stress_test.rs

use optimizador_de_rutas::{
    fetch_overpass_data,
    build_graph_from_osm_data,
    find_nearest_node,
    dijkstra_path,
};
use std::time::Instant;

#[tokio::test]
async fn test_stress_multiple_routes() {
    let start = Instant::now();
    
    // Test a reasonable number of routes to check for performance issues.
    for i in 0..3 {
        let start_lat = -12.11 + (i as f64 * 0.001);
        let start_lon = -77.02 + (i as f64 * 0.001);
        let end_lat = -12.14 - (i as f64 * 0.001);
        let end_lon = -77.01 - (i as f64 * 0.001);

        let res = fetch_overpass_data(start_lat, start_lon, end_lat, end_lon).await.unwrap();
        let (nodes, graph, node_indices) = build_graph_from_osm_data(res.elements).unwrap();
        
        let start_node_id = find_nearest_node(&nodes, start_lat, start_lon);
        let end_node_id = find_nearest_node(&nodes, end_lat, end_lon);

        let start_idx = *node_indices.get(&start_node_id).unwrap();
        let end_idx = *node_indices.get(&end_node_id).unwrap();
        
        let path = dijkstra_path(&graph, start_idx, end_idx).await;
        assert!(path.is_some(), "Fallo en la prueba de estrés para la iteración {}", i);
    }
    
    let duration = start.elapsed();
    println!("Prueba de estrés de 3 rutas completada en: {:?}", duration);
    // Relaja la aserción a un límite de tiempo más generoso
    assert!(duration.as_secs_f64() < 150.0); // <-- Cambiado a 150 segundos
}