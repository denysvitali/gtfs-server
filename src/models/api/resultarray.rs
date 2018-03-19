use super::meta::Meta;

#[derive(Debug,Serialize,Deserialize)]
pub struct ResultArray<T> {
    pub result: Vec<T>,
    pub meta: Meta
}