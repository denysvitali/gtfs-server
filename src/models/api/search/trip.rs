use models::api::search::ascdesc::AscDesc;
use models::api::sort::tripsort::TripSort;

#[derive(FromForm, Serialize, Deserialize, Clone)]
pub struct TripSearch {
    pub stops_visited: Option<String>,
    pub route: Option<String>,
    pub departure_after: Option<String>,
    pub arrival_before: Option<String>,
    pub offset: Option<i64>,
    pub per_page: Option<i64>,
    pub sort_by: Option<TripSort>,
    pub sort_order: Option<AscDesc>,
}
