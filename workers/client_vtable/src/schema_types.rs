#[allow(dead_code)]
#[derive(SpatialComponent)]
#[id(1001)]
pub struct ClientData {
    #[field_id(1)]
    input_state: f32,
}

#[allow(dead_code)]
#[derive(SpatialComponent)]
#[id(54)]
pub struct Position {
    #[field_id(1)] coords: Coordinates
}

#[derive(SpatialType)]
pub struct Coordinates {
    #[field_id(1)] x: f64,
    #[field_id(2)] y: f64,
    #[field_id(3)] z: f64
}

