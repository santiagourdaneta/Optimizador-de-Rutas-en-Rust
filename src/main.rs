use optimizador_de_rutas::{fetch_overpass_data, build_graph_from_osm_data, find_nearest_node, dijkstra_path, reverse_geocode};
use std::error::Error;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        return Err("Uso: cargo run -- <coordenadas_inicio> <coordenadas_fin>".into());
    }

    let start_arg = &args[1];
    let end_arg = &args[2];

    let start_coords: Vec<f64> = start_arg.split(',').filter_map(|s| s.parse().ok()).collect();
    let end_coords: Vec<f64> = end_arg.split(',').filter_map(|s| s.parse().ok()).collect();

    if start_coords.len() != 2 || end_coords.len() != 2 {
        return Err("Formato de coordenadas inválido. Usa 'lat,lon'".into());
    }

    let (start_lat, start_lon) = (start_coords[0], start_coords[1]);
    let (end_lat, end_lon) = (end_coords[0], end_coords[1]);

    println!("Consultando Overpass API para datos de la zona...");

    let res = fetch_overpass_data(start_lat, start_lon, end_lat, end_lon).await?;

    println!("Datos de Overpass obtenidos. Procesando {} elementos.", res.elements.len());

    let (nodes, graph, node_indices) = build_graph_from_osm_data(res.elements)?;

    let start_node_id = find_nearest_node(&nodes, start_lat, start_lon);
    let end_node_id = find_nearest_node(&nodes, end_lat, end_lon);

    if let (Some(&start_idx), Some(&end_idx)) = (node_indices.get(&start_node_id), node_indices.get(&end_node_id)) {
        println!("Calculando la ruta con Dijkstra...");

        if let Some((total_distance, predecessors)) = dijkstra_path(&graph, start_idx, end_idx).await {
            println!("Ruta más corta encontrada. Distancia total: {:.2} km", total_distance);

            let mut path_indices = Vec::new();
            let mut current_idx = end_idx;
            while current_idx != start_idx {
                path_indices.push(current_idx);
                current_idx = *predecessors.get(&current_idx).unwrap();
            }
            path_indices.push(start_idx);
            path_indices.reverse();

            println!("\nDetalle de la ruta:");

            let start_node_data = nodes.get(&start_node_id).unwrap();
            let start_address = reverse_geocode(start_node_data.lat, start_node_data.lon).await.unwrap_or_else(|_| "Dirección no encontrada".to_string());
            println!("  Inicio: Lat: {}, Lon: {} -> {}", start_node_data.lat, start_node_data.lon, start_address);

            let end_node_data = nodes.get(&end_node_id).unwrap();
            let end_address = reverse_geocode(end_node_data.lat, end_node_data.lon).await.unwrap_or_else(|_| "Dirección no encontrada".to_string());
            println!("  Fin: Lat: {}, Lon: {} -> {}", end_node_data.lat, end_node_data.lon, end_address);

        } else {
            println!("No se pudo encontrar una ruta entre los puntos dados.");
        }
    } else {
        println!("No se pudieron encontrar nodos de inicio o fin cercanos en los datos de OSM.");
    }

    Ok(())
}