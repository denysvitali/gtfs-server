#![feature(plugin)]
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
mod models;
mod routes;

use r2d2_postgres::{TlsMode, PostgresConnectionManager};
use r2d2::Pool;

use models::stop::Stop;
use routes::RoutesHandler;
use routes::api;

use chrono::NaiveTime;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate num_derive;

fn create_pool() -> Pool<PostgresConnectionManager> {
    let manager = PostgresConnectionManager::new(
        "postgres://postgres:mysecretpassword@172.18.0.2:5432",
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
        api::stops::stops,
        api::stops::stops_by_id,
        api::stops::stops_near_default,
        api::stops::stops_near,
        api::stops::stops_by_trip,
        api::trips::trips_stopid,
        api::trips::trip,
        api::times::times_trip,
        api::times::times_stop
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