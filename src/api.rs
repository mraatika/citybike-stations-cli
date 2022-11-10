use std::collections::HashMap;

use crate::station::{Edge, Nearest, Station, StationDto, StationNode, StationResponse};
use reqwest;
use reqwest::header::{ACCEPT, CONTENT_TYPE};

fn edge_to_station(edge: &Edge) -> Station {
    let node: &StationNode = &edge.node;
    let place: &StationDto = &node.place;

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

fn map_response_to_stations(response: &StationResponse) -> Vec<Station> {
    let nearest: &Nearest = &response.data.nearest;
    let edges: &Vec<Edge> = &nearest.edges;
    edges.into_iter().map(|e| edge_to_station(&e)).collect()
}

fn create_payload() -> HashMap<String, String> {
    let query = "{
        nearest(lat:60.24020252949141, lon: 25.10188542043851, filterByPlaceTypes:[BICYCLE_RENT]) {
            edges {
                node {
                    id
                    distance
                    place {
                        ... on BikeRentalStation {
                            stationId
                            name
                            bikesAvailable
                            spacesAvailable
                            lat
                            lon
                            allowDropoff
                        }
                    }
                }
            }
        }
    }";
    let mut payload = HashMap::new();
    payload.insert(String::from("query"), String::from(query));
    payload
}

pub fn do_fetch(url: &str) -> Result<StationResponse, reqwest::Error> {
    let payload = create_payload();

    reqwest::blocking::Client::new()
        .post(url)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .json(&payload)
        .send()?
        .json::<StationResponse>()
}

pub fn fetch_stations(url: &str) -> Vec<Station> {
    let response = do_fetch(&url);

    match response {
        Ok(station_response) => map_response_to_stations(&station_response),
        Err(err) => {
            println!("Err {:?}", err);
            Vec::<Station>::with_capacity(0)
        }
    }
}
