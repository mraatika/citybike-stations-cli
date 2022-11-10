use station_table::print_table;

use crate::api::fetch_stations;

pub mod api;
pub mod station;
pub mod station_table;

const URL: &str = "https://api.digitransit.fi/routing/v1/routers/hsl/index/graphql";

fn main() {
    let stations = fetch_stations(&URL);
    print_table(stations);
    println!("");
    println!("Tiedot haettu: {URL}");
}
