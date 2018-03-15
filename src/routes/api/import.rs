use super::super::RoutesHandler;
use super::super::Pool;
use super::super::PostgresConnectionManager;

use super::super::Json;
use super::super::State;
use super::model_api::successresult::SuccessResult;

use super::super::super::importer;


#[get("/import/stops/<feed_id>")]
pub fn stops(rh: State<RoutesHandler>, feed_id: String) -> Json<SuccessResult> {
    importer::create_tables(&rh.pool);
    importer::parse_stops(&feed_id, "./resources/gtfs/sbb/stops.txt", &rh.pool);
    let sr = SuccessResult { success: true };
    Json(sr)
}