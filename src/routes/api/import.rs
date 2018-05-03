//! `/import` related routes

extern crate zip;

use super::super::Pool;
use super::super::PostgresConnectionManager;
use super::super::RoutesHandler;

use super::super::Json;
use super::super::State;
use super::model_api::successresult::SuccessResult;

use self::zip::ZipArchive;
use super::super::super::importer;
use std::fs::File;
use std::thread;

#[get("/import/url/<feed_url>")]
pub fn url(rh: State<RoutesHandler>, feed_url: String) -> Json<SuccessResult> {
    let pool: Pool<PostgresConnectionManager> = rh.pool.clone();
    thread::spawn(move || {
        //importer::create_tables(&pool);
        let zar = importer::download_feed_zip(&feed_url, &pool);
        if zar.is_some() {
            let mut zar = zar.unwrap();
            importer::parse_feed_zip(&mut zar, &pool)
        } else {
            println!("Unable to get the feed zip");
        }
    });
    let sr = SuccessResult { success: true };
    Json(sr)
}

#[get("/import/fs/<file_name>")]
pub fn fs(rh: State<RoutesHandler>, file_name: String) -> Json<SuccessResult> {
    let pool: Pool<PostgresConnectionManager> = rh.pool.clone();
    thread::spawn(move || {
        //importer::create_tables(&pool);
        importer::parse_feed_zip(
            &mut ZipArchive::new(File::open("resources/gtfs/flixbus/flixbus.zip").unwrap())
                .unwrap(),
            &pool,
        );
    });
    let sr = SuccessResult { success: true };
    Json(sr)
}

#[get("/import/agency/<feed_id>")]
pub fn agency(rh: State<RoutesHandler>, feed_id: String) -> Json<SuccessResult> {
    let pool: Pool<PostgresConnectionManager> = rh.pool.clone();
    thread::spawn(move || {
        importer::create_tables(&pool);
        //importer::parse_agency(&feed_id, "./resources/gtfs/sbb/agency.txt", &pool);
        println!("Agency parsed!");
    });
    let sr = SuccessResult { success: true };
    Json(sr)
}

#[get("/import/stops/<feed_id>")]
pub fn stops(rh: State<RoutesHandler>, feed_id: String) -> Json<SuccessResult> {
    let pool: Pool<PostgresConnectionManager> = rh.pool.clone();
    thread::spawn(move || {
        importer::create_tables(&pool);
        //importer::parse_stops(&feed_id, "./resources/gtfs/sbb/stops.txt", &pool);
        println!("Stops parsed!");
    });
    let sr = SuccessResult { success: true };
    Json(sr)
}

#[get("/import/times/<feed_id>")]
pub fn times(rh: State<RoutesHandler>, feed_id: String) -> Json<SuccessResult> {
    let pool: Pool<PostgresConnectionManager> = rh.pool.clone();
    thread::spawn(move || {
        importer::create_tables(&pool);
        //importer::parse_stop_times(&feed_id, "./resources/gtfs/sbb/stop_times.txt", &pool);
        println!("Stop Times parsed!");
    });
    let sr = SuccessResult { success: true };
    Json(sr)
}

#[get("/import/routes/<feed_id>")]
pub fn routes(rh: State<RoutesHandler>, feed_id: String) -> Json<SuccessResult> {
    let pool: Pool<PostgresConnectionManager> = rh.pool.clone();
    thread::spawn(move || {
        importer::create_tables(&pool);
        //importer::parse_routes(&feed_id, "./resources/gtfs/sbb/routes.txt", &pool);
        println!("Routes parsed!");
    });
    let sr = SuccessResult { success: true };
    Json(sr)
}

#[get("/import/trips/<feed_id>")]
pub fn trips(rh: State<RoutesHandler>, feed_id: String) -> Json<SuccessResult> {
    let pool: Pool<PostgresConnectionManager> = rh.pool.clone();
    thread::spawn(move || {
        importer::create_tables(&pool);
        //importer::parse_trips(&feed_id, "./resources/gtfs/sbb/trips.txt", &pool);
        println!("Trips parsed!");
    });
    let sr = SuccessResult { success: true };
    Json(sr)
}

#[get("/import/calendar/<feed_id>")]
pub fn calendar(rh: State<RoutesHandler>, feed_id: String) -> Json<SuccessResult> {
    let pool: Pool<PostgresConnectionManager> = rh.pool.clone();
    thread::spawn(move || {
        importer::create_tables(&pool);
        //importer::parse_calendar(&feed_id, "./resources/gtfs/sbb/calendar.txt", &pool);
        println!("Calendar parsed!");
    });
    let sr = SuccessResult { success: true };
    Json(sr)
}
