use super::super::trip::Trip;
use super::meta::Meta;

#[derive(Debug,Serialize)]
pub struct TripResult {
    pub result: Vec<Trip>,
    pub meta: Meta
}