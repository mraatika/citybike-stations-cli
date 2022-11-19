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
