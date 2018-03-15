use super::super::RoutesHandler;
use super::super::Pool;
use super::super::PostgresConnectionManager;

use super::super::Json;
use super::super::State;
use super::model_api::successresult::SuccessResult;

use super::super::super::importer;
use std::thread;

#[get("/import/stops/<feed_id>")]
pub fn stops(rh: State<RoutesHandler>, feed_id: String) -> Json<SuccessResult> {
    let pool : Pool<PostgresConnectionManager> = rh.pool.clone();
    thread::spawn(move || {
        importer::create_tables(&pool);
        importer::parse_stops(&feed_id, "./resources/gtfs/sbb/stops.txt", &pool);
        println!("Stops parsed!");
    });
    let sr = SuccessResult { success: true };
    Json(sr)
}
#[get("/import/trips/<feed_id>")]
pub fn trips(rh: State<RoutesHandler>, feed_id: String) -> Json<SuccessResult> {
    let pool : Pool<PostgresConnectionManager> = rh.pool.clone();
    thread::spawn(move || {
        importer::create_tables(&pool);
        importer::parse_trips(&feed_id, "./resources/gtfs/sbb/trips.txt", &pool);
        println!("Trips parsed!");
    });
    let sr = SuccessResult { success: true };
    Json(sr)
}