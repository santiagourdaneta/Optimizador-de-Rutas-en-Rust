// File: src/lib.rs

use std::collections::{HashMap, BinaryHeap};
use std::cmp::Ordering;
use petgraph::prelude::*;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OsmNode {
    pub id: i64,
    #[serde(rename = "lat")]
    pub lat: f64,
    #[serde(rename = "lon")]
    pub lon: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OsmWay {
    pub id: i64,
    pub nodes: Vec<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OsmResponse {
    pub elements: Vec<serde_json::Value>,
}

#[derive(Copy, Clone, PartialEq)]
pub struct State {
    pub cost: f64,
    pub position: NodeIndex,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.partial_cmp(&self.cost).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

impl Eq for State {}

pub async fn fetch_overpass_data(
    start_lat: f64,
    start_lon: f64,
    end_lat: f64,
    end_lon: f64,
) -> Result<OsmResponse, Box<dyn Error>> {
    let min_lat = start_lat.min(end_lat) - 0.05;
    let max_lat = start_lat.max(end_lat) + 0.05;
    let min_lon = start_lon.min(end_lon) - 0.05;
    let max_lon = start_lon.max(end_lon) + 0.05;

    let bbox = format!("{min_lat},{min_lon},{max_lat},{max_lon}");
    let query = format!("[out:json];(way[highway]({bbox});node(w););out body;>;out skel qt;");
    
    let client = reqwest::Client::new();
    let res = client.post("https://overpass-api.de/api/interpreter")
        .body(query)
        .send()
        .await?
        .json::<OsmResponse>()
        .await?;
    Ok(res)
}

pub fn build_graph_from_osm_data(
    elements: Vec<serde_json::Value>,
) -> Result<(HashMap<i64, OsmNode>, UnGraph<i64, f64>, HashMap<i64, NodeIndex>), Box<dyn Error>> {
    let mut nodes: HashMap<i64, OsmNode> = HashMap::new();
    let mut graph = UnGraph::<i64, f64>::new_undirected();
    let mut node_indices: HashMap<i64, NodeIndex> = HashMap::new();

    for elem in &elements {
        if elem["type"] == "node" {
            let node_data: OsmNode = serde_json::from_value(elem.clone())?;
            let id = node_data.id;
            nodes.insert(id, node_data);
            let idx = graph.add_node(id);
            node_indices.insert(id, idx);
        }
    }
    
    for elem in &elements {
        if elem["type"] == "way" {
            let way: OsmWay = serde_json::from_value(elem.clone())?;
            for i in 0..way.nodes.len() - 1 {
                let node1_id = way.nodes[i];
                let node2_id = way.nodes[i+1];
                if let (Some(&idx1), Some(&idx2)) = (node_indices.get(&node1_id), node_indices.get(&node2_id)) {
                    let node1_data = nodes.get(&node1_id).ok_or("Node not found")?;
                    let node2_data = nodes.get(&node2_id).ok_or("Node not found")?;
                    let distance = haversine(node1_data.lat, node1_data.lon, node2_data.lat, node2_data.lon);
                    graph.add_edge(idx1, idx2, distance);
                }
            }
        }
    }
    Ok((nodes, graph, node_indices))
}

pub async fn reverse_geocode(lat: f64, lon: f64) -> Result<String, Box<dyn Error>> {
    let url = format!("https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat={}&lon={}&countrycodes=pe", lat, lon);
    let client = reqwest::Client::new();
    let response = client.get(&url)
        .header("User-Agent", "optimizador-de-rutas-app/1.0")
        .send()
        .await?;
    let json_data: serde_json::Value = response.json().await?;
    if let Some(address) = json_data["display_name"].as_str() {
        Ok(address.to_string())
    } else {
        Ok(format!("Dirección no encontrada para {}, {}", lat, lon))
    }
}

pub async fn dijkstra_path(
    graph: &UnGraph<i64, f64>,
    start: NodeIndex,
    end: NodeIndex
) -> Option<(f64, HashMap<NodeIndex, NodeIndex>)> {
    let mut dist: HashMap<NodeIndex, f64> = HashMap::new();
    let mut predecessors: HashMap<NodeIndex, NodeIndex> = HashMap::new();
    let mut heap = BinaryHeap::new();
    dist.insert(start, 0.0);
    heap.push(State { cost: 0.0, position: start });

    while let Some(State { cost, position }) = heap.pop() {
        if position == end {
            return Some((*dist.get(&end).unwrap(), predecessors));
        }
        if cost > *dist.get(&position).unwrap_or(&f64::MAX) {
            continue;
        }
        for edge in graph.edges(position) {
            let next = edge.target();
            let new_cost = cost + edge.weight();
            if new_cost < *dist.get(&next).unwrap_or(&f64::MAX) {
                dist.insert(next, new_cost);
                predecessors.insert(next, position);
                heap.push(State { cost: new_cost, position: next });
            }
        }
    }
    None
}

pub fn find_nearest_node(nodes: &HashMap<i64, OsmNode>, lat: f64, lon: f64) -> i64 {
    let mut nearest_id = 0;
    let mut min_distance = f64::MAX;
    
    for (id, node) in nodes.iter() {
        let distance = haversine(lat, lon, node.lat, node.lon);
        if distance < min_distance {
            min_distance = distance;
            nearest_id = *id;
        }
    }
    nearest_id
}

pub fn haversine(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    const R: f64 = 6371.0; 
    let d_lat = (lat2 - lat1).to_radians();
    let d_lon = (lon2 - lon1).to_radians();
    let lat1 = lat1.to_radians();
    let lat2 = lat2.to_radians();
    let a = (d_lat / 2.0).sin() * (d_lat / 2.0).sin() +
            lat1.cos() * lat2.cos() *
            (d_lon / 2.0).sin() * (d_lon / 2.0).sin();
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    R * c
}
