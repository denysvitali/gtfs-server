//! This is the documentation for `gtfs-server`
//!

#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;

extern crate chrono;
extern crate num_traits;
extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate regex;
extern crate rocket_contrib;
#[macro_use] extern crate runtime_fmt;

mod test;

mod importer;
pub mod models;
pub mod routes;

use r2d2::Pool;
use r2d2_postgres::{PostgresConnectionManager, TlsMode};

use rocket::response::NamedFile;
use routes::api;
use routes::ui;
use routes::RoutesHandler;
use std::env;
use std::path::{Path, PathBuf};

use chrono::{NaiveDate, NaiveTime};
use std::{time,thread};
use models::api::result::Result;
use postgres::Error;
use importer::update_db;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate num_derive;

fn create_pool() -> Pool<PostgresConnectionManager> {
    let mut hostname = "172.18.0.2";
    let mut password = String::from("mysecretpassword");
    let MAX_ATTEMPTS = 10;
    let mut attempts = 1;

    if env::var_os("IN_DOCKER").is_some()
        && env::var_os("IN_DOCKER").unwrap().to_str().unwrap() == "true"
    {
        hostname = "gtfs-db.service.dc1.consul";
        password = String::from(
            env::var_os("POSTGRES_PASSWORD")
                .unwrap()
                .to_str()
                .unwrap()
                .clone(),
        );
    }

    let connection_string = format!("postgres://postgres:{}@{}:5432", password, hostname);

    let manager = PostgresConnectionManager::new(
        connection_string.clone(),
        TlsMode::None).unwrap();
    let mut pool = Pool::new(manager);

    while pool.is_err() && attempts <= MAX_ATTEMPTS {
        let manager = PostgresConnectionManager::new(
            connection_string.clone(),
            TlsMode::None).unwrap();

        println!("Unable to connect, attempt {}/{}", attempts, MAX_ATTEMPTS);
        pool = Pool::new(manager);
        attempts += 1;
        println!("Waiting 5s for the next try...");
        thread::sleep_ms(5000);
    }

    if pool.is_ok() {
        return pool.unwrap();
    }
    panic!(format!("Unable to connect to {}!", hostname));
}

#[get("/css/<file..>")]
fn static_css(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/dist/css/").join(file)).ok()
}

#[get("/js/<file..>")]
fn static_js(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/dist/js/").join(file)).ok()
}

fn start_server(rh: RoutesHandler) {
    rocket::ignite()
        .manage(rh)
        .mount(
            "/api",
            routes![
                api::main,
                api::import::agency,
                api::import::stops,
                api::import::trips,
                api::import::calendar,
                api::import::routes,
                api::import::times,
                api::import::url,
                api::import::fs,
                api::db::update,
                api::db::version,
                api::agency::agency_by_id,
                api::agency::agency,
                api::routes::routes,
                api::routes::routes_by_query,
                api::routes::route_by_stop_uid,
                api::routes::route_by_id,
                api::routes::route_by_bbox,
                api::stops::stops,
                api::stops::stops_by_id,
                api::stops::stops_near_default,
                api::stops::stops_near,
                api::stops::stops_by_trip,
                api::stops::stops_in_bbox,
                api::stops::stops_in_bbox_radius,
                api::stops::stops_latlng_test,
                api::stops::stops_latlng_test_zoom,
                api::stop_times::stop_times_after_near,
                api::stop_times::stop_times_between_near,
                api::stop_times::stop_times_by_stop_after,
                api::stop_times::stop_times_by_stop_between,
                api::trips::trips,
                api::trips::trips_stopid,
                api::trips::trip,
                api::trips::trips_by_route,
                api::trips::trips_by_query,
                api::trips::trips_by_bbox,
                api::trips::trips_by_bbox_query,
                api::times::times_query,
                api::times::times_by_trip,
                api::times::times_stop,
                api::times::times_stop_query
            ],
        )
        .mount("/", routes![ui::import::main])
        .mount("/", routes![static_css, static_js])
        .launch();
}

fn main() {
    let pool = create_pool();

    println!("Updating Database...");
    update_db(&pool.clone());

    let rh = RoutesHandler { pool };

    //create_tables(&pool);

    //let feed_id = parse_feed("./resources/gtfs/sbb/feed_info.txt", &pool);
    //println!("{}", feed_id);
    /*parse_agency(
        &feed_id,
        "./resources/gtfs/sbb/agency.txt",
        &conn
    );*/

    //stops_near(&pool, 46.00598, 8.952449, 200.0);

    start_server(rh);
    //parse_stops(&feed_id, "./resources/gtfs/sbb/stops.txt", &conn);
    //parse_routes(&feed_id, "./resources/gtfs/sbb/routes.txt", &conn);
    //parse_trips(&feed_id, "./resources/gtfs/sbb/trips.txt", &conn);
    //parse_stop_times(&feed_id, "./resources/gtfs/sbb/stop_times.txt", &pool);
}
