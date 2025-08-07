# Optimizador de Rutas en Rust 🗺️

Este proyecto es una herramienta de línea de comandos (CLI) escrita en **Rust** que utiliza la **Overpass API de OpenStreetMap** para encontrar la ruta más corta entre dos puntos geográficos. El algoritmo de **Dijkstra** se implementa para calcular el camino óptimo a través de la red de calles.

## 🚀 Características

-   **Ruteo inteligente**: Utiliza un grafo no dirigido para representar la red de calles, donde cada nodo es un punto de interés y cada arista es un tramo de calle ponderado por su distancia.
-   **Precisión geográfica**: Emplea la fórmula de Haversine para calcular distancias exactas entre puntos.
-   **Integración con Overpass**: Consulta directamente la base de datos de OSM para obtener datos actualizados de la zona de interés.
-   **Información detallada**: Proporciona la distancia total del trayecto y las direcciones de inicio y fin (a través de la API de Nominatim).

## 🛠️ Requisitos

-   **Rust** y **Cargo** instalados. Puedes instalarlos [aquí](https://www.rust-lang.org/tools/install).
-   Una conexión a Internet activa para consultar las APIs.

## 🏃 Modo de uso

1.  Clona el repositorio:
    ```bash
    git clone https://github.com/santiagourdaneta/Optimizador-de-Rutas-en-Rust/
    cd Optimizador-de-Rutas-en-Rust
    ```
2.  Ejecuta el programa pasando las coordenadas de inicio y fin como argumentos en el formato `lat,lon`.

    Por ejemplo, para encontrar la ruta entre dos puntos en Lima, Perú:
    ```bash
    cargo run -- "-12.11797,-76.98541" "-12.10000,-76.99000"
    ```

3.  El programa imprimirá en la consola los detalles de la ruta encontrada, incluyendo la distancia total y las direcciones.

## 🧪 Pruebas

El proyecto incluye pruebas unitarias y de integración para garantizar el correcto funcionamiento de los componentes clave.

Para ejecutar todas las pruebas, usa el siguiente comando:
```bash
cargo test

📄 Licencia
Este proyecto está bajo la licencia MIT.

Labels: rust, cli-tool, openstreetmap, dijkstra, pathfinding, geo-spatial, logistics

Tags: rust-lang, overpass-api, petgraph, routing, shortest-path

Hashtags: #RustLang #OpenStreetMap #Pathfinding #CLItool #Dijkstra #GeoSpatial
