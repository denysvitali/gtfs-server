use super::meta::Meta;

#[derive(Debug,Serialize)]
pub struct Result<T> {
    pub result: T,
    pub meta: Meta
}