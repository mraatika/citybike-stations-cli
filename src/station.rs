use std::fmt::{self, write};

use serde::{Deserialize, Serialize};

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
