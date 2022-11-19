use crate::api::fetch_stations;
use station_table::print_table;
use std::env;

pub mod api;
pub mod station;
pub mod station_table;

// 60.24020252949141 25.10188542043851

fn main() {
    const URL: &str = "https://api.digitransit.fi/routing/v1/routers/hsl/index/graphql";

    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        panic!("Latitude and longitude are needed!");
    }

    let lat = &args[1];
    let lon = &args[2];

    println!("Searching stations with coordinates {lat}, {lon}");

    let stations = fetch_stations(&URL, &lat, &lon);
    print_table(stations);
    println!("");
    println!("Tiedot haettu: {URL}");
}
