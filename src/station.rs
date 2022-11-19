use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Station {
    pub station_id: String,
    pub name: String,
    pub bikes_available: u32,
    pub spaces_available: u32,
    pub lat: f32,
    pub lon: f32,
    pub allow_dropoff: bool,
    pub distance: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StationDto {
    pub station_id: String,
    pub name: String,
    pub bikes_available: u32,
    pub spaces_available: u32,
    pub lat: f32,
    pub lon: f32,
    pub allow_dropoff: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StationNode {
    pub id: String,
    pub distance: u32,
    pub place: StationDto,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Edge {
    pub node: StationNode,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Nearest {
    pub edges: Vec<Edge>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StationResponseData {
    pub nearest: Nearest,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StationResponse {
    pub data: StationResponseData,
}

fn edge_to_station(edge: Edge) -> Station {
    let node: StationNode = edge.node;
    let place: StationDto = node.place;

    Station {
        station_id: place.station_id.clone(),
        name: place.name.clone(),
        bikes_available: place.bikes_available,
        spaces_available: place.spaces_available,
        lat: place.lat,
        lon: place.lon,
        distance: node.distance,
        allow_dropoff: place.allow_dropoff,
    }
}

pub fn response_to_stations(response: StationResponse) -> Vec<Station> {
    let edges: Vec<Edge> = response.data.nearest.edges;
    edges.into_iter().map(|e| edge_to_station(e)).collect()
}
