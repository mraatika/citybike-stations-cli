use std::collections::HashMap;

use crate::station::Station;
use crate::station_response::StationResponse;

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

pub fn fetch_stations(url: &str, lat: &str, lon: &str) -> Vec<Station> {
    let payload = create_payload(&lat, &lon);
    let response = do_fetch(&url, &payload);

    match response {
        Ok(station_response) => station_response.get_stations(),
        Err(err) => {
            println!("Err {:?}", err);
            Vec::<Station>::with_capacity(0)
        }
    }
}
