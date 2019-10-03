//! `/stop_times` related routes

use super::model_api::error::Error;
use super::model_api::meta::Meta;
use super::model_api::result::Result;
use super::model_api::resultarray::ResultArray;
use super::model_api::stopdistance::StopDistance;

use models::boundingbox::BoundingBox;
use models::stop::Stop;

use rocket_contrib::json::Json;
use super::super::Pool;
use super::super::PostgresConnectionManager;
use super::super::RoutesHandler;
use super::super::State;
use postgres::row::Row;
use std::f64;

use rocket::http::ContentType;
use rocket::response::content;

use chrono::NaiveDate;
use chrono::NaiveTime;
use chrono::ParseError;
use chrono::ParseResult;
use models::api::stoptime::StopTime;
use models::api::stoptimes::StopTimes;
use models::api::stoptimes::TripTime;
use models::coordinate::Coordinate;
use postgres::NoTls;
use std::collections::HashMap;
use std::str::FromStr;

/// `/stop_times/after/<time>/near/<lat>/<lng>/<radius>`
/// Gets an array of [StopTimes](../../../models/api/stoptimes/struct.StopTimes.html) after a `<time>`,
/// within a `<radius>` from a specified location (`<lat>`, `<lng>`).
/// Returns a [ResultArray](../../../models/api/result/struct.ResultArray.html)<[StopTimes](../../../models/api/stoptimes/struct.StopTimes.html)>
#[get("/stop_times/after/<time>/near/<lat>/<lng>/<radius>")]
pub fn stop_times_after_near(
    rh: State<RoutesHandler>,
    time: String,
    lat: f64,
    lng: f64,
    radius: f64,
) -> Json<ResultArray<StopTimes>> {
    let query = r#"SELECT DISTINCT
    trip.uid,
    stop.uid,
    j1.departure_time, j1.uid
    FROM stop_time, stop, trip
    INNER JOIN LATERAL (
        SELECT s2.uid, st2.departure_time FROM stop_time as st2
         JOIN stop as s2 ON s2.id = st2.stop_id AND s2.feed_id = st2.feed_id
        WHERE st2.trip_id = stop_time.trip_id
        AND st2.stop_sequence = (stop_time.stop_sequence + 1)
    )  j1 ON true
    WHERE
        stop_time.stop_id = stop.id AND
        stop_time.feed_id = stop.feed_id AND
        trip.trip_id = stop_time.trip_id AND
        trip.feed_id = stop_time.feed_id AND
        ST_Distance(stop."position", ST_GeomFromText($1)) < $2 AND
        stop_time.departure_time >= $3
    LIMIT 8000"#;

    let mut conn = rh.pool.clone().get().unwrap();
    let point = format!("POINT({} {})", lng, lat);
    let stop_times = conn.query(query, &[&point, &radius, &time]);

    let stop_times: Vec<StopTimes> = parse_stop_times(&stop_times.expect("Query failed"));

    if stop_times.len() == 0 {
        return Json(ResultArray {
            result: None,
            meta: Meta {
                success: true,
                error: None,
                pagination: None,
            },
        });
    }

    Json(ResultArray {
        result: Some(stop_times),
        meta: Meta {
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
pub fn stop_times_between_near(
    rh: State<RoutesHandler>,
    time: String,
    time2: String,
    lat: f64,
    lng: f64,
    radius: f64,
) -> Json<ResultArray<StopTimes>> {
    let query = r#"SELECT DISTINCT
    trip.uid,
    stop.uid,
    j1.departure_time, j1.uid
    FROM stop_time, stop, trip
    INNER JOIN LATERAL (
        SELECT s2.uid, st2.departure_time FROM stop_time as st2
         JOIN stop as s2 ON s2.id = st2.stop_id AND s2.feed_id = st2.feed_id
        WHERE st2.trip_id = stop_time.trip_id
        AND st2.stop_sequence = (stop_time.stop_sequence + 1)
    )  j1 ON true
    WHERE
        stop_time.stop_id = stop.id AND
        stop_time.feed_id = stop.feed_id AND
        trip.trip_id = stop_time.trip_id AND
        trip.feed_id = stop_time.feed_id AND
        ST_Distance(stop."position", ST_GeomFromText($1)) < $2 AND
        stop_time.departure_time >= $3 AND
        stop_time.departure_time < $4
    LIMIT 8000"#;

    let mut conn = rh.pool.clone().get().unwrap();
    let point = format!("POINT({} {})", lng, lat);
    let stop_times = conn.query(query, &[&point, &radius, &time, &time2]);

    let stop_times: Vec<StopTimes> = parse_stop_times(&stop_times.expect("Query failed"));

    Json(ResultArray {
        result: Some(stop_times),
        meta: Meta {
            success: true,
            error: None,
            pagination: None,
        },
    })
}

#[get("/stop_times/between/<time>/<time2>/in/<bbox>")]
pub fn stop_times_between_in(
    rh: State<RoutesHandler>,
    time: String,
    time2: String,
    bbox: BoundingBox,
) -> Json<ResultArray<StopTimes>> {
    let query = r#"SELECT DISTINCT
    trip.uid,
    stop.uid,
    j1.departure_time, j1.uid
    FROM stop_time, stop, trip
    INNER JOIN LATERAL (
        SELECT s2.uid, st2.departure_time FROM stop_time as st2
         JOIN stop as s2 ON s2.id = st2.stop_id AND s2.feed_id = st2.feed_id
        WHERE st2.trip_id = stop_time.trip_id
        AND st2.stop_sequence = (stop_time.stop_sequence + 1)
    )  j1 ON true
    WHERE
        stop_time.stop_id = stop.id AND
        stop_time.feed_id = stop.feed_id AND
        trip.trip_id = stop_time.trip_id AND
        trip.feed_id = stop_time.feed_id AND
        ST_Contains(
            ST_MakeEnvelope(
                $2,
                $1,
                $4,
                $3,
                4326),
            position::geometry
        ) AND
        stop_time.departure_time >= $5 AND
        stop_time.departure_time < $6
    LIMIT 1000"#;

    let mut conn = rh.pool.clone().get().unwrap();
    let p1 = bbox.p1;
    let p2 = bbox.p2;
    let stop_times = conn.query(
        query,
        &[
            &p1.lat,
            &p1.lng,
            &p2.lat,
            &p2.lng,
            &time,
            &time2,
        ],
    );

    let stop_times: Vec<StopTimes> = parse_stop_times(&stop_times.expect("Query failed"));

    Json(ResultArray {
        result: Some(stop_times),
        meta: Meta {
            success: true,
            error: None,
            pagination: None,
        },
    })
}

/*

*/

/// `/stop_times/by-stop/<stop>/after/<time>`
/// Gets an array of [StopTimes](../../../models/api/stoptimes/struct.StopTimes.html) at a specified `<stop>`
/// after `<time>`
/// Returns a [ResultArray](../../../models/api/result/struct.ResultArray.html)<[StopTimes](../../../models/api/stoptimes/struct.StopTimes.html)>
#[get("/stop_times/by-stop/<stop>/after/<time>")]
pub fn stop_times_by_stop_after(
    rh: State<RoutesHandler>,
    stop: String,
    time: String,
) -> Json<Result<StopTimes>> {
    let query = r#"SELECT DISTINCT trip.uid,
    stop.uid,
    j1.departure_time, j1.uid
    FROM stop_time, stop, trip
    INNER JOIN LATERAL (
        SELECT s2.uid, st2.departure_time FROM stop_time as st2
         JOIN stop as s2 ON s2.id = st2.stop_id AND s2.feed_id = st2.feed_id
        WHERE st2.trip_id = stop_time.trip_id
        AND st2.stop_sequence = (stop_time.stop_sequence + 1)
    )  j1 ON true
    WHERE stop_time.stop_id = stop.id
    AND stop.uid = $1
    AND stop_time.feed_id = stop.feed_id
    AND trip.trip_id = stop_time.trip_id AND trip.feed_id = stop_time.feed_id
    AND stop_time.departure_time > $2
    ORDER BY stop.uid, j1.departure_time
    LIMIT 5000"#;

    let mut conn = rh.pool.clone().get().unwrap();
    let stop_times = conn.query(query, &[&stop, &time]);

    let stop_times: Vec<StopTimes> = parse_stop_times(&stop_times.expect("Query failed"));

    if (&stop_times).len() == 0 {
        return Json(Result {
            result: None,
            meta: Meta {
                success: true,
                error: None,
                pagination: None,
            },
        });
    }

    let st_result: StopTimes = stop_times.get(0).unwrap().clone();

    Json(Result {
        result: Some(st_result),
        meta: Meta {
            success: true,
            error: None,
            pagination: None,
        },
    })
}

/// `/stop_times/by-stop/<stop>/between/<time>/<time2>/<date>`
/// Gets an array of [StopTimes](../../../models/api/stoptimes/struct.StopTimes.html) between
/// `<time>` and `<time2>` at a specified `<stop>`.
/// Returns a [ResultArray](../../../models/api/result/struct.ResultArray.html)<[StopTimes](../../../models/api/stoptimes/struct.StopTimes.html)>
#[get("/stop_times/by-stop/<stop>/between/<time>/<time2>/<date>")]
pub fn stop_times_by_stop_between(
    rh: State<RoutesHandler>,
    stop: String,
    time: String,
    time2: String,
    date: String,
) -> Json<Result<StopTimes>> {
    let query = r#"SELECT DISTINCT
    trip.uid,
    stop.uid,
    j1.departure_time, j1.uid
    FROM stop_time, stop, trip
    INNER JOIN LATERAL (
        SELECT s2.uid, st2.departure_time FROM stop_time as st2
         JOIN stop as s2 ON s2.id = st2.stop_id AND s2.feed_id = st2.feed_id
        WHERE st2.trip_id = stop_time.trip_id
        AND st2.stop_sequence = (stop_time.stop_sequence + 1)
    )  j1 ON true
    WHERE
        stop_time.stop_id = stop.id AND
        stop_time.feed_id = stop.feed_id AND
        trip.trip_id = stop_time.trip_id AND
        trip.feed_id = stop_time.feed_id AND
        stop.uid = $1 AND
        stop_time.departure_time >= $2 AND
        stop_time.departure_time < $3 AND
        (
       	    trip.service_id IN (
       	        SELECT calendar.service_id
       	        FROM calendar
       	        WHERE calendar.feed_id=trip.feed_id AND
       	        calendar.start_date <= $4 AND
       	        calendar.end_date >= $4
       	    ) OR
       	    trip.service_id IN (
       	        SELECT calendar_date.service_id
       	        FROM calendar_date
       	        WHERE calendar_date.feed_id = trip.feed_id AND
       	        calendar_date."date" = $4
       	    )
       )
    LIMIT 8000"#;

    let mut conn = rh.pool.clone().get().unwrap();
    let stop_times = conn.query(
        query,
        &[&stop, &time, &time2, &date],
    );

    let stop_times: Vec<StopTimes> = parse_stop_times(&stop_times.expect("Query failed"));

    if stop_times.len() == 1 {
        Json(Result {
            result: Some(stop_times.get(0).unwrap().clone()),
            meta: Meta {
                success: true,
                error: None,
                pagination: None,
            },
        })
    } else {
        Json(Result {
            result: Option::None,
            meta: Meta {
                success: true,
                error: None,
                pagination: None,
            },
        })
    }
}

fn parse_stop_times(rows: &Vec<Row>) -> Vec<StopTimes> {
    let mut stop_times: Vec<StopTimes> = Vec::new();
    let mut stop_times_hm: HashMap<String, Vec<TripTime>> = HashMap::new();

    for i in rows {
        let stop_uid: String = i.get(1);

        if !stop_times_hm.contains_key(&stop_uid) {
            &mut stop_times_hm.insert(stop_uid.clone(), Vec::new());
        }

        let mut st_stop = stop_times_hm.get_mut(&stop_uid).unwrap();

        match NaiveTime::parse_from_str(i.get(2), "%H:%M:%S") {
            Ok(t) => {
                (st_stop).push(TripTime {
                    trip: i.get(0),
                    time: t,
                    next_stop: i.get(3),
                });
            }
            ParseError => {
                continue;
            }
        }
    }
    for (k, v) in &stop_times_hm {
        stop_times.push(StopTimes {
            stop: k.to_string(),
            time: v.to_vec(),
        })
    }
    stop_times
}
