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
extern crate rocket_contrib;
extern crate regex;

mod test;

mod importer;
pub mod models;
pub mod routes;

use r2d2::Pool;
use r2d2_postgres::{PostgresConnectionManager, TlsMode};

use routes::api;
use routes::ui;
use routes::RoutesHandler;
use std::env;
use std::path::{Path, PathBuf};
use rocket::response::NamedFile;

use chrono::{NaiveDate, NaiveTime};

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate num_derive;

fn create_pool() -> Pool<PostgresConnectionManager> {
    let mut hostname = "172.18.0.2";
    let mut password = String::from("mysecretpassword");

    if env::var_os("IN_DOCKER").is_some()
        && env::var_os("IN_DOCKER").unwrap().to_str().unwrap() == "true"
    {
        hostname = "postgres";
        password = String::from(
            env::var_os("POSTGRES_PASSWORD")
                .unwrap()
                .to_str()
                .unwrap()
                .clone(),
        );
    }

    let connection_string = format!("postgres://postgres:{}@{}:5432", password, hostname);

    let manager = PostgresConnectionManager::new(connection_string, TlsMode::None).unwrap();
    let pool = Pool::new(manager).unwrap();
    pool
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
                api::trips::trips,
                api::trips::trips_stopid,
                api::trips::trip,
                api::trips::trips_by_route,
                api::trips::trips_by_query,
                api::trips::trips_by_bbox,
                api::times::times_query,
                api::times::times_by_trip,
                api::times::times_stop,
                api::times::times_stop_query
            ],
        )
        .mount("/",
        routes![
            ui::import::main
        ])
        .mount("/",
        routes![
            static_css,
            static_js
        ])
        .launch();
}

fn main() {
    let pool = create_pool();
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
