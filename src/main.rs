use crate::api::fetch_stations;
use station_table::print_table;
use std::env;

pub mod address;
pub mod api;
pub mod station;
pub mod station_table;

// 60.24020252949141 25.10188542043851

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Search term is missing!");
    }

    let search_term = &args[1];

    if search_term.is_empty() {
        println!("Search term is empty, nothing to be done!");
    } else {
        let stations = fetch_stations(&search_term);
        print_table(stations);
    }
}
