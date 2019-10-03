use rocket::http::RawStr;
use rocket::request::FromFormValue;

#[derive(Serialize, Deserialize, Clone, PartialEq, FromFormValue)]
pub enum AscDesc {
    ASC,
    DESC,
}

impl AscDesc {
    pub fn as_str(&self) -> &str {
        match self {
            &AscDesc::ASC => "ASC",
            &AscDesc::DESC => "DESC",
        }
    }
}