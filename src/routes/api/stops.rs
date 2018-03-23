//! `/stops` related routes

use super::model_api::result::Result;
use super::model_api::resultarray::ResultArray;
use super::model_api::stopdistance::StopDistance;
use super::model_api::meta::Meta;
use super::model_api::error::Error;

use models::stop::Stop;

use super::super::RoutesHandler;
use super::super::Json;
use super::super::State;
use super::super::Pool;
use super::super::PostgresConnectionManager;
use postgres::rows::Row;

/// `/stops`  
/// Returns a [ResultArray](../../../models/api/resultarray/struct.ResultArray.html)<[Stop](../../../models/stop/struct.Stop.html)>
///  
/// ### Example
/// `/api/stops/` returns:  
/**  
    ```json
    {
      "result": [
        {
          "uid": "s-f8f952-anzolachiesa",
          "name": "Anzola, chiesa",
          "lat": 45.989902,
          "lng": 8.345061,
          "location_type": 0
        },
        {
          "uid": "s-90980f-bognancotvillaelda",
          "name": "Bognanco, T. Villa Elda",
          "lat": 46.122295,
          "lng": 8.2107725,
          "location_type": 0
        },
        (...)
      ],
      "meta": {
        "success": true
      }
    }
    ```
**/
#[get("/stops")]
pub fn stops(rh: State<RoutesHandler>) -> Json<ResultArray<Stop>> {

    let sr = ResultArray::<Stop> {
        result: Some(get_stops(&rh.pool)),
        meta: Meta{
            success: true,
            error: Option::None
        }
    };
    Json(sr)
}

/// `/stops/<stop_id>`  
/// Gets a single [Stop](../../../models/stop/struct.Stop.html) from its `stop_id`.  
/// Returns a [Result](../../../models/api/result/struct.Result.html)<[Stop](../../../models/stop/struct.Stop.html)>
#[get("/stops/<stop_id>")]
pub fn stops_by_id(rh: State<RoutesHandler>, stop_id: String) -> Json<Result<Stop>> {

    let stop = get_stop_by_id(stop_id, &rh.pool);
    if stop.is_none() {
        return Json(Result::<Stop> {
            result: Option::None,
            meta: Meta {
                success: false,
                error: Some(Error {
                    code: 2,
                    message: String::from("Invalid stop id")
                })
            }
        });
    }

    let sr = Result::<Stop> {
        result: stop,
        meta: Meta{
            success: true,
            error: Option::None
        }
    };
    Json(sr)
}
/// `/stops/by-trip/<trip_id>`  
/// get the [Stop](../../../models/stop/struct.Stop.html)s visited by a [Trip](../../../models/trip/struct.Trip.html) uid.  
/// Returns a [ResultArray](../../../models/api/resultarray/struct.ResultArray.html)<[Stop](../../../models/stop/struct.Stop.html)>
#[get("/stops/by-trip/<trip_id>")]
pub fn stops_by_trip(rh: State<RoutesHandler>, trip_id: String) -> Json<ResultArray<Stop>> {

    let sr = ResultArray::<Stop> {
        result: Some(get_stops_by_trip(trip_id, &rh.pool)),
        meta: Meta{
            success: true,
            error: Option::None
        }
    };
    Json(sr)
}

/// `/stops/near/<lat>/<lng>`  
/// Gets an array of [StopDistance](../../../models/api/stopdistance/struct.StopDistance.html)s,
/// within 100.0 meters from <lat>,<lng> - nearest first.  
/// Returns a [ResultArray](../../../models/api/resultarray/struct.ResultArray.html)
/// <[StopDistance](../../../models/api/stopdistance/struct.StopDistance.html)>
#[get("/stops/near/<lat>/<lng>")]
pub fn stops_near_default(rh: State<RoutesHandler>, lat: f32, lng: f32) -> Json<ResultArray<StopDistance>> {

    let sr = ResultArray::<StopDistance> {
        result: Some(get_stops_near(&rh.pool, lat, lng, 100.0)),
        meta: Meta{
            success: true,
            error: Option::None
        }
    };
    Json(sr)
}

/// `/stops/near/<lat>/<lng>/<meters>`  
/// Gets an array of [StopDistance](../../../models/api/stopdistance/struct.StopDistance.html)s,
/// within <meters> meters from <lat>,<lng>
/// nearest first, of Stops near the provided coordinate.  
/// Returns a [ResultArray](../../../models/api/resultarray/struct.ResultArray.html)
/// <[StopDistance](../../../models/api/stopdistance/struct.StopDistance.html)>
#[get("/stops/near/<lat>/<lng>/<meters>")]
pub fn stops_near(rh: State<RoutesHandler>, lat: f32, lng: f32, meters: f64) -> Json<ResultArray<StopDistance>> {

    let sr = ResultArray::<StopDistance> {
        result: Some(get_stops_near(&rh.pool, lat, lng, meters)),
        meta: Meta{
            success: true,
            error: Option::None
        }
    };
    Json(sr)
}

fn parse_stop_row(row: &Row) -> Stop {
    let lat : f64 = row.get(6);
    let lng : f64 = row.get(7);

    let uid = row.get(0);
    let id = row.get(1);
    let name = row.get(2);
    let location_type = row.get(3);
    let parent_station: Option<String> = row.get(4);
    let feed_id = row.get(5);

    let mut stop = Stop::new(
        uid, name, lat, lng, location_type, parent_station
    );

    &stop.set_id(id);
    &stop.set_feed_id(feed_id);

    stop
}

fn get_stop_by_id(stop_id: String, pool: &Pool<PostgresConnectionManager>) -> Option<Stop> {
    let query = "SELECT
        uid,
        id,
        name,
        type,
        (SELECT s.uid FROM stop as s WHERE s.id = stop.parent_stop AND s.feed_id = stop.feed_id) as parent_stop,
        feed_id,
        ST_Y(position::geometry) as lat,
        ST_X(position::geometry) as lng FROM stop
        WHERE uid=$1
        LIMIT 1";

    let conn = pool.clone().get().unwrap();
    let stops = conn.query(
        query, &[&stop_id]
    );

    let stops = &stops.expect("Query failed");

    if stops.len() != 1 {
        return Option::None;
    }

    let stop = parse_stop_row(&stops.get(0));
    Some(stop)
}

fn get_stops(pool: &Pool<PostgresConnectionManager>) -> Vec<Stop> {
    let query = "SELECT
        uid,
        id,
        name,
        type,
        (SELECT s.uid FROM stop as s WHERE s.id = stop.parent_stop AND s.feed_id = stop.feed_id) as parent_stop,
        feed_id,
        ST_Y(position::geometry) as lat,
        ST_X(position::geometry) as lng FROM stop
        LIMIT 50";

    let conn = pool.clone().get().unwrap();
    let stops = conn.query(
        query, &[]
    );

    let mut stops_result : Vec<Stop> = Vec::new();

    for row in stops.expect("Query failed").iter() {
        let stop = parse_stop_row(&row);
        stops_result.push(stop);
    }

    stops_result
}

fn get_stops_near(pool: &Pool<PostgresConnectionManager>,
                  lat: f32,
                  lng: f32,
                  meters: f64) -> Vec<StopDistance> {
    let query = "SELECT * FROM (SELECT
        uid,
        id,
        name,
        type,
        (SELECT s.uid FROM stop as s WHERE s.id = stop.parent_stop AND s.feed_id = stop.feed_id) as parent_stop,
        feed_id,
        ST_Y(position::geometry) as lat,
        ST_X(position::geometry) as lng,
        ST_Distance(position, \
        ST_GeomFromText($1)) as distance \
        FROM stop) as s1 WHERE \
        distance <= $2\
        ORDER BY distance ASC \
        LIMIT 50;";

    //println!(format!("{}", query));
    let conn = pool.clone().get().unwrap();
    let stops = conn.query(
        query,
        &[
            &format!("POINT({:.5} {:.5})", lng, lat),
            &meters
        ]
    );

    let mut stops_result : Vec<StopDistance> = Vec::new();

    for row in stops.expect("Query failed").iter() {
        let stop = parse_stop_row(&row);
        let distance = row.get(8);

        let sd = StopDistance {
            stop,
            distance
        };

        stops_result.push(sd);
    }

    stops_result
}

fn get_stops_by_trip(trip_id: String, pool: &Pool<PostgresConnectionManager>) -> Vec<Stop> {

    /*
        	stop_time.stop_sequence, \
            stop_time.arrival_time, \
            stop_time.departure_time \
    */

    let query = "SELECT \
	stop.uid, \
	stop.id, \
	stop.name, \
	stop.type, \
	(SELECT stop.uid FROM stop as s WHERE s.id = stop.parent_stop AND s.feed_id = stop.feed_id) as parent_stop, \
	stop.feed_id, \
	ST_Y(position::geometry) as lat, \
    ST_X(position::geometry) as lng \

    FROM trip \
    INNER JOIN stop_time ON (trip.trip_id = stop_time.trip_id) \
    INNER JOIN stop ON (stop_time.stop_id = stop.id) \
    WHERE \
            trip.feed_id = stop_time.feed_id \
            AND \
            stop_time.feed_id = stop.feed_id \
            AND \
            trip.uid=$1 \
    ORDER BY stop_time.stop_sequence ASC;";

    let conn = pool.clone().get().unwrap();
    let stops = conn.query(
        query, &[&trip_id]
    );

    let mut stops_result : Vec<Stop> = Vec::new();

    for row in stops.expect("Query failed").iter() {
        let stop = parse_stop_row(&row);

        stops_result.push(stop);
    }

    stops_result



}