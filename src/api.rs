use std::collections::HashMap;

use crate::station::{Edge, Station, StationDto, StationNode, StationResponse};
use reqwest;
use reqwest::header::{ACCEPT, CONTENT_TYPE};

fn create_payload(lat: &str, lon: &str) -> HashMap<String, String> {
    let query = format!(
        "{{
        nearest(lat: {lat}, lon: {lon}, filterByPlaceTypes:[BICYCLE_RENT]) {{
            edges {{
                node {{
                    id
                    distance
                    place {{
                        ... on BikeRentalStation {{
                            stationId
                            name
                            bikesAvailable
                            spacesAvailable
                            lat
                            lon
                            allowDropoff
                        }}
                    }}
                }}
            }}
        }}
    }}"
    );

    let mut payload = HashMap::new();
    payload.insert(String::from("query"), String::from(query));
    payload
}

pub fn do_fetch(
    url: &str,
    payload: &HashMap<String, String>,
) -> Result<StationResponse, reqwest::Error> {
    reqwest::blocking::Client::new()
        .post(url)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .json(&payload)
        .send()?
        .json::<StationResponse>()
}

fn edge_to_station(edge: Edge) -> Station {
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

fn map_response_to_stations(response: StationResponse) -> Vec<Station> {
    let edges: Vec<Edge> = response.data.nearest.edges;
    edges.into_iter().map(|e| edge_to_station(e)).collect()
}

pub fn fetch_stations(url: &str, lat: &str, lon: &str) -> Vec<Station> {
    let payload = create_payload(&lat, &lon);
    let response = do_fetch(&url, &payload);

    match response {
        Ok(station_response) => map_response_to_stations(station_response),
        Err(err) => {
            println!("Err {:?}", err);
            Vec::<Station>::with_capacity(0)
        }
    }
}
