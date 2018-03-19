#[derive(Debug, Serialize,Deserialize)]
pub struct Error {
    pub code : i32,
    pub message : String
}