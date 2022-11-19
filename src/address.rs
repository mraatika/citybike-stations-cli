use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AddressProperties {
    pub id: String,
    pub name: String,
    pub confidence: u32,
    pub localadmin: String,
    pub label: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Geometry {
    pub coordinates: [f32; 2],
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddressFeature {
    pub geometry: Geometry,
    pub properties: AddressProperties,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddressResponse {
    pub features: Vec<AddressFeature>,
}

pub struct Coords(pub f32, pub f32);

pub fn get_coords_from_address(response: &AddressResponse) -> Coords {
    let feature = &response.features[0];
    let geometry = &feature.geometry;
    Coords(geometry.coordinates[1], geometry.coordinates[0])
}
