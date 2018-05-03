use importer::serde::{Deserialize, Deserializer};
use importer::serde_de;
use std::str::FromStr;

#[derive(Debug, Deserialize)]
pub struct TripCSV {
    pub route_id: String,
    pub service_id: String,
    pub trip_id: String,
    pub trip_headsign: Option<String>,
    pub trip_short_name: Option<String>,
    pub direction_id: Option<i32>,
    pub block_id: Option<String>,
    pub shape_id: Option<String>,
    pub wheelchair_accessible: Option<String>,
    pub bikes_allowed: Option<String>,
}

fn i32_def<'de, D>(des: D) -> Result<Option<i32>, D::Error>
where
    D: Deserializer<'de>,
{
    println!("Deserializing");
    match String::deserialize(des) {
        Ok(v) => {
            let value = v.parse::<i32>();
            if value.is_ok() {
                return Ok(Some(value.unwrap()));
            } else {
                return Ok(Some(0));
            }
        }
        Err(e) => Ok(Option::None),
    }
}
