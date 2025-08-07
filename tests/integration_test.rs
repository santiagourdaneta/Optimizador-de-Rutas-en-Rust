use optimizador_de_rutas::*;
use std::collections::HashMap;
use petgraph::graph::UnGraph;

#[tokio::test]
async fn test_full_route_calculation() {
    let start_lat = -12.115;
    let start_lon = -77.027;
    let end_lat = -12.148;
    let end_lon = -77.018;

    let res = fetch_overpass_data(start_lat, start_lon, end_lat, end_lon).await.unwrap();
    assert!(!res.elements.is_empty(), "The Overpass API returned no data for the test area.");

    let (nodes, graph, node_indices) = build_graph_from_osm_data(res.elements).unwrap();
    assert!(!nodes.is_empty(), "No nodes were created from the OSM data.");
    assert!(graph.edge_count() > 0, "No edges were created in the graph.");

    let start_node_id = find_nearest_node(&nodes, start_lat, start_lon);
    let end_node_id = find_nearest_node(&nodes, end_lat, end_lon);

    let start_idx = *node_indices.get(&start_node_id).unwrap();
    let end_idx = *node_indices.get(&end_node_id).unwrap();

    let path = dijkstra_path(&graph, start_idx, end_idx).await;
    assert!(path.is_some(), "Could not find a route between the test points.");
}