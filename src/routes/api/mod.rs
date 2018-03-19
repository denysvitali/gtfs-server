use super::model_api;
use super::super::NaiveTime;
pub mod stops;
pub mod import;
pub mod trips;
pub mod times;

#[get("/")]
pub fn main() -> String {
    return String::from("gtfs-server");
}