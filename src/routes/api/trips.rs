use super::model_api::meta::Meta;

use models::trip::Trip;
use super::model_api::result::Result;
use super::model_api::resultarray::ResultArray;

use super::super::RoutesHandler;
use super::super::Json;
use super::super::State;
use super::super::Pool;
use super::super::PostgresConnectionManager;

use postgres::rows::Row;

#[get("/trips/by-stop/<stop_id>")]
pub fn trips_stopid(rh: State<RoutesHandler>, stop_id: String) -> Json<ResultArray<Trip>>{
    let query =
        "SELECT \
        uid,\
        route_id,\
        service_id,\
        trip_id,\
        headsign,\
        short_name,\
        direction_id,\
        feed_id \
        FROM trip \
        WHERE trip_id IN \
        (SELECT trip_id FROM stop_time WHERE \
            stop_id=(SELECT stop.id FROM stop WHERE uid=$1) \
            GROUP BY trip_id \
        ) LIMIT 50";

    let conn = rh.pool.clone().get().unwrap();
    let trips = conn.query(
        query,
        &[
            &stop_id
        ]
    );

    let mut trips_result: Vec<Trip> = Vec::new();

    for row in trips.expect("Query failed").iter() {
        let route = parse_trip_row(&row);
        trips_result.push(route);
    }

    let rr = ResultArray::<Trip> {
        result: trips_result,
        meta: Meta{
            success: true,
            error: Option::None
        }
    };

    Json(rr)
}

#[get("/trips/<trip_id>")]
pub fn trip(rh: State<RoutesHandler>, trip_id: String) -> Json<Result<Trip>>{
    let query =
        "SELECT \
        uid,\
        route_id,\
        service_id,\
        trip_id,\
        headsign,\
        short_name,\
        direction_id,\
        feed_id \
        FROM trip \
        WHERE uid = $1";

    let conn = rh.pool.clone().get().unwrap();
    let trips = conn.query(
        query,
        &[
            &trip_id
        ]
    );

    let trip = parse_trip_row(&(trips).unwrap().get(0));

    let result = Result::<Trip> {
        result: trip,
        meta: Meta {
            success: true,
            error: Option::None
        }
    };

    Json(result)
}

fn parse_trip_row(row: &Row) -> Trip {
    let mut t = Trip::new(
        row.get(0),
        row.get(1),
        row.get(2),
        row.get(4),
        row.get(5),
        row.get(6),
    );
    t.set_id(row.get(3));
    t.set_feed_id(row.get(7));
    t
}