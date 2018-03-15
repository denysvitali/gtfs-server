// https://opentransportdata.swiss/en/dataset/timetable-2018-gtfs/permalink
extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;

mod importer;
mod models;

use r2d2_postgres::{TlsMode, PostgresConnectionManager};
use r2d2::Pool;

use models::stop::Stop;

#[macro_use]
extern crate serde_derive;


fn create_pool() -> Pool<PostgresConnectionManager> {
    let manager = PostgresConnectionManager::new(
        "postgres://postgres:mysecretpassword@172.18.0.2:5432",
        TlsMode::None
    ).unwrap();
    let pool = Pool::new(manager).unwrap();
    pool
}


fn stops_near(pool: &Pool<PostgresConnectionManager>, lat: f32, lng: f32, meters: f64){
    let query = "SELECT 
        id, 
        name, 
        type, 
        parent_stop, 
        feed_id,
        ST_Y(position::geometry) as lat,
        ST_X(position::geometry) as lng FROM stop WHERE \
        ST_Distance(position, \
        ST_GeomFromText($1)) <= $2;";

    //println!(format!("{}", query));
    let conn = pool.clone().get().unwrap();
    let stops = conn.query(
        query,
    &[
            &format!("POINT({:.5} {:.5})", lng, lat),
            &meters
        ]
    );

    for row in stops.expect("Query failed").iter() {
        //let a : String = row.get(2);
        let lat : f64 = row.get(5);
        let lng : f64 = row.get(6);

        let stop = Stop {
            id: row.get(0),
            name: row.get(1),
            lat: lat,
            lng: lng,
            location_type: row.get(2),
            parent_station: row.get(3),
            feed_id: row.get(4)

        };
        println!("{:?}", stop);
    }
}

fn main() {
    let pool = create_pool();
    //create_tables(&pool);

    //let feed_id = parse_feed("./resources/gtfs/sbb/feed_info.txt", &pool);
    //println!("{}", feed_id);
    /*parse_agency(
        &feed_id,
        "./resources/gtfs/sbb/agency.txt",
        &conn
    );*/

    stops_near(&pool, 46.00598, 8.952449, 200.0);
    //parse_stops(&feed_id, "./resources/gtfs/sbb/stops.txt", &conn);
    //parse_routes(&feed_id, "./resources/gtfs/sbb/routes.txt", &conn);
    //parse_trips(&feed_id, "./resources/gtfs/sbb/trips.txt", &conn);
    //parse_stop_times(&feed_id, "./resources/gtfs/sbb/stop_times.txt", &pool);
}
