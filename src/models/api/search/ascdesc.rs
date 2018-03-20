pub enum AscDesc {
    ASC,
    DESC
}

impl AscDesc {
    pub fn as_str(&self) -> &str {
        match self {
            &AscDesc::ASC => "ASC",
            &AscDesc::DESC => "DESC"
        }
    }
}