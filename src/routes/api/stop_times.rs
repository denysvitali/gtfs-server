//! `/stop_times` related routes

use super::model_api::error::Error;
use super::model_api::meta::Meta;
use super::model_api::result::Result;
use super::model_api::resultarray::ResultArray;
use super::model_api::stopdistance::StopDistance;

use models::boundingbox::BoundingBox;
use models::stop::Stop;

use super::super::Json;
use super::super::Pool;
use super::super::PostgresConnectionManager;
use super::super::RoutesHandler;
use super::super::State;
use postgres::rows::Row;
use std::f64;

use rocket::http::ContentType;
use rocket::response::content;

use models::coordinate::Coordinate;
use models::api::stoptime::StopTime;
use postgres::rows::Rows;
use models::api::stoptimes::StopTimes;
use std::collections::HashMap;
use chrono::NaiveTime;
use models::api::stoptimes::TripTime;
use std::str::FromStr;
use chrono::ParseError;
use chrono::ParseResult;

/// `/stop_times/after/<time>/near/<lat>/<lng>/<radius>`
/// Gets an array of [StopTimes](../../../models/api/stoptimes/struct.StopTimes.html) after a `<time>`,
/// within a `<radius>` from a specified location (`<lat>`, `<lng>`).
/// Returns a [ResultArray](../../../models/api/result/struct.ResultArray.html)<[StopTimes](../../../models/api/stoptimes/struct.StopTimes.html)>
#[get("/stop_times/after/<time>/near/<lat>/<lng>/<radius>")]
pub fn stop_times_after_near(rh: State<RoutesHandler>,
                             time: String,
                             lat: f64,
                             lng: f64,
                             radius: f64
)
    -> Json<ResultArray<StopTimes>> {
    let query = r#"SELECT DISTINCT
    trip.uid,
    stop.uid,
    stop_time.departure_time
    FROM stop_time, stop,trip
    WHERE
        stop_time.stop_id = stop.id AND
        stop_time.feed_id = stop.feed_id AND
        trip.trip_id = stop_time.trip_id AND
        trip.feed_id = stop_time.feed_id AND
        ST_Distance(stop."position", ST_GeomFromText($1)) < $2 AND
        stop_time.departure_time >= $3
    LIMIT 8000"#;

    let time1 : ParseResult<NaiveTime> = NaiveTime::from_str(&time);

    if time1.is_err() {
        return Json(ResultArray{
            result: None,
            meta: Meta{
                success: false,
                error: Some(Error {
                    code: 1,
                    message: String::from("Time is invalid"),
                }),
                pagination: None,
            },
        })
    }

    let conn = rh.pool.clone().get().unwrap();
    let point = format!("POINT({} {})", lng, lat);
    let stop_times = conn.query(query, &[&point, &radius, &time1.unwrap()]);

    let stop_times : Vec<StopTimes> = parse_stop_times(&stop_times.expect("Query failed"));

    if stop_times.len() == 0 {
        return Json(ResultArray {
            result: None,
            meta: Meta{
                success: true,
                error: None,
                pagination: None,
            },
        });
    }

    Json(ResultArray{
        result: Some(stop_times),
        meta: Meta{
            success: true,
            error: None,
            pagination: None,
        },
    })
}

/// `/stop_times/after/<time>/near/<lat>/<lng>/<radius>`
/// Gets an array of [StopTimes](../../../models/api/stoptimes/struct.StopTimes.html) after a `<time>`,
/// within a `<radius>` from a specified location (`<lat>`, `<lng>`).
/// Returns a [ResultArray](../../../models/api/result/struct.ResultArray.html)<[StopTimes](../../../models/api/stoptimes/struct.StopTimes.html)>
#[get("/stop_times/between/<time>/<time2>/near/<lat>/<lng>/<radius>")]
pub fn stop_times_between_near(rh: State<RoutesHandler>,
                             time: String,
                             time2: String,
                             lat: f64,
                             lng: f64,
                             radius: f64
)
                             -> Json<ResultArray<StopTimes>> {
    let query = r#"SELECT DISTINCT
    trip.uid,
    stop.uid,
    stop_time.departure_time
    FROM stop_time, stop,trip
    WHERE
        stop_time.stop_id = stop.id AND
        stop_time.feed_id = stop.feed_id AND
        trip.trip_id = stop_time.trip_id AND
        trip.feed_id = stop_time.feed_id AND
        ST_Distance(stop."position", ST_GeomFromText($1)) < $2 AND
        stop_time.departure_time >= $3 AND
        stop_time.departure_time < $4
    LIMIT 8000"#;

    let time1 : ParseResult<NaiveTime> = NaiveTime::from_str(&time);
    let time2 : ParseResult<NaiveTime> = NaiveTime::from_str(&time2);

    if time1.is_err() {
        return Json(ResultArray{
            result: None,
            meta: Meta{
                success: false,
                error: Some(Error {
                    code: 1,
                    message: String::from("Time 1 is invalid"),
                }),
                pagination: None,
            },
        })
    }

    if time2.is_err() {
        return Json(ResultArray{
            result: None,
            meta: Meta{
                success: false,
                error: Some(Error {
                    code: 1,
                    message: String::from("Time 2 is invalid"),
                }),
                pagination: None,
            },
        })
    }

    let conn = rh.pool.clone().get().unwrap();
    let point = format!("POINT({} {})", lng, lat);
    let stop_times = conn.query(query, &[&point,
        &radius,
        &time1.unwrap(),
        &time2.unwrap()]
    );

    let stop_times : Vec<StopTimes> = parse_stop_times(&stop_times.expect("Query failed"));

    if stop_times.len() == 0 {
        return Json(ResultArray {
            result: None,
            meta: Meta{
                success: true,
                error: None,
                pagination: None,
            },
        });
    }

    Json(ResultArray{
        result: Some(stop_times),
        meta: Meta{
            success: true,
            error: None,
            pagination: None,
        },
    })
}

fn parse_stop_times(rows: &Rows) -> Vec<StopTimes> {
    let mut stop_times : Vec<StopTimes> = Vec::new();
    let mut stop_times_hm : HashMap<String, Vec<TripTime>> = HashMap::new();

    for i in rows {

        let stop_uid : String = i.get(1);

        if !stop_times_hm.contains_key(&stop_uid) {
            &mut stop_times_hm.insert(stop_uid.clone(), Vec::new());
        }

        let mut st_stop = stop_times_hm.get_mut(&stop_uid).unwrap();

        (st_stop).push(TripTime{
            trip: i.get(0),
            time: i.get(2),
        });
    }
    for (k,v) in &stop_times_hm{
        stop_times.push(StopTimes{
            stop: k.to_string(),
            time: v.to_vec(),
        })
    }
    stop_times
}
