//! This is the documentation for `gtfs-server`
//!

#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;

extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate rocket_contrib;
extern crate num_traits;
extern crate chrono;

mod test;

mod importer;
pub mod models;
pub mod routes;

use r2d2_postgres::{TlsMode, PostgresConnectionManager};
use r2d2::Pool;

use models::stop::Stop;
use routes::RoutesHandler;
use routes::api;
use std::env;
use std::ffi::OsString;

use chrono::{NaiveTime, NaiveDate};

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate num_derive;

fn create_pool() -> Pool<PostgresConnectionManager> {
    let mut os_str : Option<OsString>;
    let mut os : &OsString;
    let mut hostname = "172.18.0.2";
    let mut password = String::from("mysecretpassword");

    if env::var_os("IN_DOCKER").is_some() && 
        env::var_os("IN_DOCKER").unwrap().to_str().unwrap() == "true" {
        hostname = "postgres";
        password = String::from(
            env::var_os("POSTGRES_PASSWORD").unwrap().to_str().unwrap().clone());
    }


    let connection_string = format!("postgres://postgres:{}@{}:5432",
        password,
        hostname
    );

    let manager = PostgresConnectionManager::new(
        connection_string,
        TlsMode::None
    ).unwrap();
    let pool = Pool::new(manager).unwrap();
    pool
}

fn start_server(rh : RoutesHandler) {
    rocket::ignite()
        .manage(rh)
        .mount("/api", routes![
        api::main,
        api::import::agency,
        api::import::stops,
        api::import::trips,
        api::import::calendar,
        api::import::routes,
        api::import::times,
        api::agency::agency_by_id,
        api::routes::routes,
        api::routes::route_by_id,
        api::stops::stops,
        api::stops::stops_by_id,
        api::stops::stops_near_default,
        api::stops::stops_near,
        api::stops::stops_by_trip,
        api::trips::trips,
        api::trips::trips_stopid,
        api::trips::trip,
        api::times::times_trip,
        api::times::times_stop,
        api::times::times_stop_query
    ]).launch();
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

    let rocket = start_server(rh);
    //parse_stops(&feed_id, "./resources/gtfs/sbb/stops.txt", &conn);
    //parse_routes(&feed_id, "./resources/gtfs/sbb/routes.txt", &conn);
    //parse_trips(&feed_id, "./resources/gtfs/sbb/trips.txt", &conn);
    //parse_stop_times(&feed_id, "./resources/gtfs/sbb/stop_times.txt", &pool);
}