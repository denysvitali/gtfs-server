//! Bouding Box struct and implementation
use models::coordinate::Coordinate;

#[derive(Debug, Serialize)]
pub struct BoundingBox {
    pub p1: Coordinate,
    pub p2: Coordinate,
}
