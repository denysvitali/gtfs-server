use importer::serde_de;
use std::str::FromStr;
use importer::serde::{de, Deserialize, Deserializer};

#[derive(Debug,Deserialize)]
pub struct CalendarCSV {
    pub service_id: String,
    #[serde(deserialize_with="bool_des")]
    pub monday: bool,
    #[serde(deserialize_with="bool_des")]
    pub tuesday: bool,
    #[serde(deserialize_with="bool_des")]
    pub wednesday: bool,
    #[serde(deserialize_with="bool_des")]
    pub thursday: bool,
    #[serde(deserialize_with="bool_des")]
    pub friday: bool,
    #[serde(deserialize_with="bool_des")]
    pub saturday: bool,
    #[serde(deserialize_with="bool_des")]
    pub sunday: bool,
    pub start_date: String,
    pub end_date: String
}

fn bool_des<'de, D> (des : D) -> Result<bool, D::Error> where
    D: Deserializer<'de> {

    match String::deserialize(des) {
        Ok(v) => Ok(v == "1"),
        Err(e) => Err(e)
    }
}