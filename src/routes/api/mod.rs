use super::model_api;
use super::super::{NaiveTime, NaiveDate};

pub mod agency;
pub mod import;
pub mod routes;
pub mod stops;
pub mod trips;
pub mod times;

#[get("/")]
pub fn main() -> String {
    return String::from("gtfs-server");
}