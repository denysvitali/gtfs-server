use rocket::request::FromFormValue;
use rocket::http::RawStr;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
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

impl<'v> FromFormValue<'v> for AscDesc {
    type Error = ();

    fn from_form_value(form_value: &RawStr) -> Result<Self, <Self as FromFormValue>::Error> {
        Ok(match form_value.to_lowercase().as_str() {
            "asc" => AscDesc::ASC,
            "desc" => AscDesc::DESC,
            _ => AscDesc::ASC
        })
    }
}