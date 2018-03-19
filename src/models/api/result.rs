use super::meta::Meta;

#[derive(Debug,Serialize, Deserialize)]
pub struct Result<T> {
    pub result: T,
    pub meta: Meta
}