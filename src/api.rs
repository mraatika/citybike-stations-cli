use std::collections::HashMap;

use reqwest;
use reqwest::header::{ACCEPT, CONTENT_TYPE};

use crate::station::{Edge, Nearest, Station, StationDto, StationNode, StationResponse};

const URL: &str = "https://api.digitransit.fi/routing/v1/routers/hsl/index/graphql";
const QUERY: &str = "{
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
    let mut payload = HashMap::new();
    payload.insert(String::from("query"), String::from(QUERY));
    payload
}

pub async fn fetch_stations() {
    println!("Fetching stations from {}", URL);

    let payload = create_payload();

    match reqwest::Client::new()
        .post(URL)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .json(&payload)
        .send()
        .await
    {
        Ok(response) => match response.json::<StationResponse>().await {
            Ok(station_response) => {
                let stations = map_response_to_stations(&station_response);

                for station in stations {
                    println!("{:?}", station);
                    println!("---");
                }
            }
            Err(err) => {
                panic!("JSON parse error! {:?}", err);
            }
        },

        Err(err) => {
            panic!("Oh no! Request failed {:?}", err);
        }
    };
}
