use crate::api::fetch_stations;

pub mod api;
pub mod station;

#[tokio::main]
async fn main() {
    println!("Starting rusty engines...");
    fetch_stations().await;
}
