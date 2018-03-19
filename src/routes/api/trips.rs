use super::model_api::meta::Meta;

use models::trip::Trip;
use super::model_api::result::Result;
use super::model_api::resultarray::ResultArray;
use super::model_api::error::Error;

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
        t.uid,\
        r.uid,\
        c.uid,\
        trip_id,\
        headsign,\
        t.short_name,\
        direction_id,\
        t.feed_id \
        FROM trip as t \
        INNER JOIN calendar as c ON c.service_id=t.service_id \
        INNER JOIN route as r ON r.id = t.route_id \
        WHERE trip_id IN \
        (SELECT trip_id FROM stop_time WHERE \
            stop_id=(SELECT stop.id FROM stop WHERE uid=$1) \
            AND \
            feed_id = (SELECT stop.feed_id FROM stop WHERE uid=$1) \
            GROUP BY trip_id \
        ) \
        AND c.feed_id = t.feed_id \
        AND r.feed_id = t.feed_id \
        LIMIT 50";

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
        result: Some(trips_result),
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

    let trips = &trips.unwrap();

    if trips.len() == 0 {
        return Json(Result::<Trip> {
            result: Option::None,
            meta: Meta {
                success: false,
                error: Some(Error {
                    code: 1,
                    message: String::from("Trip not found")
                })
            }
        });
    }

    let trip = parse_trip_row(&(trips).get(0));

    let result = Result::<Trip> {
        result: Some(trip),
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