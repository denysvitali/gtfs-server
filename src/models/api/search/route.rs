#[derive(FromForm,Serialize,Deserialize)]
pub struct RouteSearch {
    pub stops_visited: Option<String>
}