use rocket_codegen::FromForm;

#[derive(Serialize, Deserialize, FromForm)]
pub struct RouteSearch {
    pub stops_visited: Option<String>,
}
