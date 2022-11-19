use std::collections::HashMap;

use crate::address::{get_coords_from_address, AddressResponse, Coords};
use crate::station::Station;
use crate::station::{response_to_stations, StationResponse};

use reqwest;
use reqwest::header::{ACCEPT, CONTENT_TYPE};

static SERVER_URL: &str = "https://api.digitransit.fi";

fn create_payload_for_station_query(coords: Coords) -> HashMap<String, String> {
    let query = format!(
        "{{
        nearest(lat: {}, lon: {}, filterByPlaceTypes:[BICYCLE_RENT]) {{
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
    }}",
        coords.0, coords.1
    );

    let mut payload = HashMap::new();
    payload.insert(String::from("query"), String::from(query));
    payload
}

fn send_stations_query(
    payload: &HashMap<String, String>,
) -> Result<StationResponse, reqwest::Error> {
    let url = format!("{SERVER_URL}/routing/v1/routers/hsl/index/graphql");
    reqwest::blocking::Client::new()
        .post(url)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .json(&payload)
        .send()?
        .json::<StationResponse>()
}

fn fetch_stations_with_location(coords: Coords) -> Vec<Station> {
    let payload = create_payload_for_station_query(coords);
    let response = send_stations_query(&payload);

    match response {
        Ok(station_response) => response_to_stations(station_response),
        Err(err) => {
            println!("Err {:?}", err);
            Vec::<Station>::with_capacity(0)
        }
    }
}

fn fetch_location_by_address(search_term: &str) -> Result<AddressResponse, reqwest::Error> {
    let url = format!("{SERVER_URL}/geocoding/v1/search");
    reqwest::blocking::Client::new()
        .get(&url)
        .query(&[("text", &search_term), ("size", &"1"), ("lang", &"fi")])
        .header(ACCEPT, "application/json")
        .send()?
        .json::<AddressResponse>()
}

pub fn fetch_stations(search_term: &str) -> Vec<Station> {
    let address_response = fetch_location_by_address(&search_term);

    match address_response {
        Ok(address_response) => {
            let coords = get_coords_from_address(&address_response);
            fetch_stations_with_location(coords)
        }
        Err(err) => {
            println!("Err {:?}", err);
            Vec::<Station>::with_capacity(0)
        }
    }
}
