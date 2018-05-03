use importer::serde::{Deserialize, Deserializer};
use importer::serde_de;
use std::str::FromStr;

#[derive(Debug, Deserialize)]
pub struct CalendarDateCSV {
    pub service_id: String,
    pub date: String,
    pub exception_type: i32,
}
