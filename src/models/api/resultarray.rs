use super::meta::Meta;

#[derive(Debug,Serialize)]
pub struct ResultArray<T> {
    pub result: Vec<T>,
    pub meta: Meta
}