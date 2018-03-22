//! `/trips` related routes

use super::model_api::meta::Meta;

use models::trip::Trip;
use models::stop::StopTrip;
use models::stop::Stop;

use models::pickup::PickUp;
use models::dropoff::DropOff;

use super::model_api::result::Result;
use super::model_api::resultarray::ResultArray;
use super::model_api::error::Error;

use super::super::RoutesHandler;
use super::super::Json;
use super::super::State;
use super::super::Pool;
use super::super::PostgresConnectionManager;

use chrono::NaiveTime;
use postgres::rows::Row;

use num_traits as num;

/// `/trips/`, returns a list of [Trip](../../../models/trip/struct.Trip.html)s.  
/// Returns a [ResultArray](../../../models/api/resultarray/struct.ResultArray.html)
/// <[Trip](../../../models/trip/struct.Trip.html)>
#[get("/trips")]
pub fn trips(rh: State<RoutesHandler>) -> Json<ResultArray<Trip>>{
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
        WHERE c.feed_id = t.feed_id \
        AND r.feed_id = t.feed_id \
        LIMIT 50";

    let conn = rh.pool.clone().get().unwrap();
    let trips = conn.query(
        query,
        &[]
    );

    let mut trips_result: Vec<Trip> = Vec::new();

    for row in trips.expect("Query failed").iter() {
        let sequence : Vec<StopTrip>;
        let mut route = parse_trip_row(&row);
        let route_uid = route.uid.clone();
        sequence = get_stop_trip(route_uid, &rh.pool);
        route.stop_sequence = sequence;
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

/// `/trips/by-stop/<stop_id>`, returns the [Trip](../../../models/trip/struct.Trip.html)s associated
/// to the specified [Stop](../../../models/stop/struct.Stop.html) UID, parametrized as `<stop_id>`.
/// Returns a [ResultArray](../../../models/api/resultarray/struct.ResultArray.html)
/// <[Trip](../../../models/trip/struct.Trip.html)>
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
        let sequence : Vec<StopTrip>;
        let mut route = parse_trip_row(&row);
        let route_uid = route.uid.clone();
        sequence = get_stop_trip(route_uid, &rh.pool);
        route.stop_sequence = sequence;
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

/// `/trips/<trip_id>`, returns the [Trip](../../../models/trip/struct.Trip.html)s associated
/// to the specified [Trip](../../../models/trip/struct.Trip.html) UID, parametrized as `<trip_id>`.
/// Returns a [Result](../../../models/api/result/struct.Result.html)
/// <[Trip](../../../models/trip/struct.Trip.html)>
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
    let sequence : Vec<StopTrip>;
    let mut trip = parse_trip_row(&(trips).get(0));
    let trip_uid = trip.uid.clone();
    sequence = get_stop_trip(String::from(trip_uid), &rh.pool);
    
    trip.stop_sequence = sequence;

    let result = Result::<Trip> {
        result: Some(trip),
        meta: Meta {
            success: true,
            error: Option::None
        }
    };

    Json(result)
}

fn get_stop_trip(trip_uid: String, pool: &Pool<PostgresConnectionManager>) -> Vec<StopTrip> {
    let query =
    r#"SELECT 
    stop.uid, 
    stop.name, 
    ST_Y(position::geometry) as lat, 
    ST_Y(position::geometry) as lng, 
    stop."type", 
    (SELECT stop.uid FROM stop as s WHERE s.id = stop.parent_stop AND s.feed_id = stop.feed_id) as parent_stop, 
    stop_time.stop_sequence, 
    stop_time.drop_off_type,
    stop_time.pickup_type,
    stop_time.arrival_time, 
    stop_time.departure_time 
    FROM stop_time 
    INNER JOIN stop ON stop.id = stop_time.stop_id 
    WHERE stop_time.trip_id = (SELECT trip.trip_id FROM trip WHERE trip.uid = $1) 
    AND stop.feed_id = stop_time.feed_id 
    ORDER BY stop_sequence ASC"#;
    
    let connection = pool.clone().get().unwrap();
    let stop_trips = connection.query(
        query, 
        &[&trip_uid]
    );
    
    let mut stop_trip_result : Vec<StopTrip> = Vec::new();
    
    for row in stop_trips.expect("Query failed").iter() {
        let route = parse_stop_trip_row(&row);
        stop_trip_result.push(route);
    }
    
    stop_trip_result
    
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

fn parse_stop_trip_row(row: &Row) -> StopTrip {
    let stop = Stop::new(
        row.get(0),
        row.get(1),
        row.get(2),
        row.get(3),
        row.get(4),
        row.get(5)
    );
    
    let drop_off_i : i32 = row.get(7);
    let pickup_i : i32 = row.get(8);
    
    let drop_off : DropOff = num::FromPrimitive::from_i32(drop_off_i).unwrap();
    let pickup : PickUp = num::FromPrimitive::from_i32(pickup_i).unwrap();
    
    let arrival_time : NaiveTime = row.get(9);
    let departure_time : NaiveTime = row.get(10);
    
    let mut st = StopTrip{
        stop,
        arrival_time,
        departure_time,
        stop_sequence: row.get(6),
        drop_off,
        pickup
    };
    
    st
}