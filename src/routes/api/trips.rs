//! `/trips` related routes

use super::model_api::meta::Meta;

use models::stop::Stop;
use models::stop::StopTrip;
use models::trip::Trip;

use models::dropoff::DropOff;
use models::pickup::PickUp;

use super::model_api::error::Error;
use super::model_api::result::Result;
use super::model_api::resultarray::ResultArray;

use super::super::Json;
use super::super::Pool;
use super::super::PostgresConnectionManager;
use super::super::RoutesHandler;
use super::super::State;

use chrono::NaiveTime;
use postgres::rows::Row;
use postgres::types::ToSql;

use std::ops::Deref;
use std::ops::DerefMut;

use std::collections::BTreeMap;
use std::collections::HashMap;

use models::api::search::trip::TripSearch;
use models::boundingbox::BoundingBox;
use num_traits as num;

/// `/trips/`, returns a list of [Trip](../../../models/trip/struct.Trip.html)s.  
/// Returns a [ResultArray](../../../models/api/resultarray/struct.ResultArray.html)
/// <[Trip](../../../models/trip/struct.Trip.html)>
#[get("/trips")]
pub fn trips(rh: State<RoutesHandler>) -> Json<ResultArray<Trip>> {
    let query = "SELECT \
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
    let trips = conn.query(query, &[]);

    let mut trips_result: Vec<Trip> = Vec::new();

    for row in trips.expect("Query failed").iter() {
        let sequence: Vec<StopTrip>;
        let mut route = parse_trip_row(&row);
        let route_uid = route.uid.clone();
        sequence = get_stop_trip(route_uid, &rh.pool);
        route.stop_sequence = Some(sequence);
        trips_result.push(route);
    }

    let rr = ResultArray::<Trip> {
        result: Some(trips_result),
        meta: Meta {
            success: true,
            error: Option::None,
        },
    };

    Json(rr)
}

/// `/trips/?query`, returns a list of [Trip](../../../models/trip/struct.Trip.html)s
/// filtered with a [TripSearch](../../../models/api/search/trip/struct.TripSearch.html) query.
/// Returns a [ResultArray](../../../models/api/resultarray/struct.ResultArray.html)
/// <[Trip](../../../models/trip/struct.Trip.html)>
///
/// Warning: The resulting [Trip](../../../models/trip/struct.Trip.html) results won't include
/// the stop_sequence vector (for a performance reason).
/// To get the related stop_sequence, make a GET request to `/trips/<uid>`.

#[get("/trips?<query>")]
pub fn trips_by_query(rh: State<RoutesHandler>, query: TripSearch) -> Json<ResultArray<Trip>> {
    let trips_result: Vec<Trip> = get_trips_by_query(&query, &rh.pool);

    let rr = ResultArray::<Trip> {
        result: Some(trips_result),
        meta: Meta {
            success: true,
            error: Option::None,
        },
    };

    Json(rr)
}

fn get_trips_by_query(ts: &TripSearch, pool: &Pool<PostgresConnectionManager>) -> Vec<Trip> {
    let mut query = String::from(
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
         AND r.feed_id = t.feed_id ",
    );

    let mut trips_result: Vec<Trip> = Vec::new();
    let mut values: Vec<String> = Vec::new();
    let mut params: Vec<&ToSql> = Vec::new();
    let mut i = 0;
    let mut addition: String;

    if ts.stops_visited.is_some() {
        addition = format!(" AND t.uid IN ( ");
        query.push_str(&addition);
        let split_stops: Vec<&str> = ts.stops_visited.as_ref().unwrap().split(",").collect();
        let mut first = true;
        for stop in split_stops {
            if first {
                first = !first;
            } else {
                addition = format!(" INTERSECT ");
                query.push_str(&addition);
            }
            i += 1;
            addition = format!(
                "SELECT
                trip.uid as tuid
                FROM trip
                INNER JOIN stop_time ON (trip.trip_id = stop_time.trip_id)
                INNER JOIN stop ON (stop_time.stop_id = stop.id)
                WHERE
                        trip.feed_id = stop_time.feed_id
                        AND
                        stop_time.feed_id = stop.feed_id
                        AND
                        stop.uid = ${}",
                &i
            );
            query.push_str(&addition);
            values.push(String::from(stop));
            println!("Stop {} ", stop);
        }
        addition = format!(" )");
        query.push_str(&addition);
    }

    for value in &values {
        params.push(value);
    }

    addition = format!(" LIMIT 50"); // TODO: Pagination
    query.push_str(&addition);

    println!("Query is: {}", query);

    let conn = pool.clone().get().unwrap();
    let trips = conn.query(&query, &params);

    for row in trips.expect("Query failed").iter() {
        //let sequence: Vec<StopTrip>;
        let mut route = parse_trip_row(&row);
        //let route_uid = route.uid.clone();
        //sequence = get_stop_trip(route_uid, pool);
        //route.stop_sequence = sequence;
        route.stop_sequence = Option::None; // stop_sequence is removed from the result
        trips_result.push(route);
    }

    trips_result
}

/// `/trips/by-stop/<stop_id>`, returns the [Trip](../../../models/trip/struct.Trip.html)s associated
/// to the specified [Stop](../../../models/stop/struct.Stop.html) UID, parametrized as `<stop_id>`.
/// Returns a [ResultArray](../../../models/api/resultarray/struct.ResultArray.html)
/// <[Trip](../../../models/trip/struct.Trip.html)>
#[get("/trips/by-stop/<stop_id>")]
pub fn trips_stopid(rh: State<RoutesHandler>, stop_id: String) -> Json<ResultArray<Trip>> {
    let query = "SELECT \
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
    let trips = conn.query(query, &[&stop_id]);

    let mut trips_result: Vec<Trip> = Vec::new();

    for row in trips.expect("Query failed").iter() {
        let sequence: Vec<StopTrip>;
        let mut route = parse_trip_row(&row);
        let route_uid = route.uid.clone();
        sequence = get_stop_trip(route_uid, &rh.pool);
        route.stop_sequence = Some(sequence);
        trips_result.push(route);
    }

    let rr = ResultArray::<Trip> {
        result: Some(trips_result),
        meta: Meta {
            success: true,
            error: Option::None,
        },
    };

    Json(rr)
}

/// `/trips/<trip_id>`, returns the [Trip](../../../models/trip/struct.Trip.html)s associated
/// to the specified [Trip](../../../models/trip/struct.Trip.html) UID, parametrized as `<trip_id>`.
/// Returns a [Result](../../../models/api/result/struct.Result.html)
/// <[Trip](../../../models/trip/struct.Trip.html)>
#[get("/trips/<trip_id>")]
pub fn trip(rh: State<RoutesHandler>, trip_id: String) -> Json<Result<Trip>> {
    let query = "SELECT \
                 trip.uid,\
                 route.uid,\
                 calendar.uid,\
                 trip.trip_id,\
                 trip.headsign,\
                 trip.short_name,\
                 trip.direction_id,\
                 trip.feed_id \
                 FROM trip, route, calendar \
                 WHERE trip.uid = $1 AND \
                 route.feed_id = trip.feed_id AND \
                 calendar.feed_id = trip.feed_id AND \
                 route.id = trip.route_id AND \
                 calendar.service_id = trip.service_id";

    let conn = rh.pool.clone().get().unwrap();
    let trips = conn.query(query, &[&trip_id]);

    let trips = &trips.unwrap();

    if trips.len() == 0 {
        return Json(Result::<Trip> {
            result: Option::None,
            meta: Meta {
                success: false,
                error: Some(Error {
                    code: 1,
                    message: String::from("Trip not found"),
                }),
            },
        });
    }
    let sequence: Vec<StopTrip>;
    let mut trip = parse_trip_row(&(trips).get(0));
    let trip_uid = trip.uid.clone();
    sequence = get_stop_trip(String::from(trip_uid), &rh.pool);

    trip.stop_sequence = Some(sequence);

    let result = Result::<Trip> {
        result: Some(trip),
        meta: Meta {
            success: true,
            error: Option::None,
        },
    };

    Json(result)
}

/// `/trips/by-route/<route_uid>`, returns the [Trip](../../../models/trip/struct.Trip.html)s associated
/// to the specified [Route](../../../models/trip/struct.Route.html) UID, parametrized as `<route_uid>`.
/// Returns a [Result](../../../models/api/result/struct.Result.html)
/// <[Trip](../../../models/trip/struct.Trip.html)>
#[get("/trips/by-route/<route_uid>")]
pub fn trips_by_route(rh: State<RoutesHandler>, route_uid: String) -> Json<ResultArray<Trip>> {
    let query = "SELECT \
        trip.uid, \
        route.uid, \
        calendar.uid, \
        trip.trip_id, \
        trip.headsign, \
        trip.short_name, \
        trip.direction_id, \
        trip.feed_id \
        FROM trip, route, calendar \
        WHERE route.uid = $1 AND \
        trip.route_id = route.id AND \
        trip.feed_id = route.feed_id AND \
        calendar.feed_id = trip.feed_id AND \
        calendar.service_id = trip.service_id
        LIMIT 50";

    let conn = rh.pool.clone().get().unwrap();
    let trips = conn.query(query, &[&route_uid]);

    let trips = &trips.unwrap();

    if trips.len() == 0 {
        return Json(ResultArray::<Trip> {
            result: Option::None,
            meta: Meta {
                success: false,
                error: Some(Error {
                    code: 1,
                    message: String::from("Trip not found"),
                }),
            },
        });
    }

    let mut trips_result: Vec<Trip> = Vec::new();

    for trip_row in trips {
        let sequence: Vec<StopTrip>;
        let mut trip = parse_trip_row(&trip_row);
        let trip_uid = trip.uid.clone();
        sequence = get_stop_trip(String::from(trip_uid), &rh.pool);
        trip.stop_sequence = Some(sequence);

        trips_result.push(trip);
    }

    let result = ResultArray::<Trip> {
        result: Some(trips_result),
        meta: Meta {
            success: true,
            error: Option::None,
        },
    };

    Json(result)
}

/// `/trips/in/<bbox>`, returns the [Trip](../../../models/trip/struct.Trip.html)s contained
/// in a [Bounding Box](../../../models/struct.BoudingBox.html).
/// Returns a [Result](../../../models/api/result/struct.Result.html)
/// <[Trip](../../../models/trip/struct.Trip.html)>
///
/// Warning: The result may contain duplicate entries!
#[get("/trips/in/<bbox>")]
pub fn trips_by_bbox(rh: State<RoutesHandler>, bbox: BoundingBox) -> Json<ResultArray<Trip>> {
    let query = r#"SELECT 
		tuid,
		ruid,
		cuid,
		ths,
		tsn,
		td,
		tfid,
		stop.uid as suid,
		stop.id as sid,
		stop."name" as sname,
		ST_Y(stop.position::geometry) as slat,
		ST_X(stop.position::geometry) as slng,
		stop."type" as st,
		pstop.uid,
		stop_time.arrival_time as st_at,
		stop_time.departure_time as st_dt,
		stop_time.stop_sequence as st_ss,
		stop_time.drop_off_type as st_do,
		stop_time.pickup_type as st_pu
	FROM 
	(SELECT 
		trip.uid as tuid,
		route.uid as ruid,
		calendar.uid as cuid,
		trip.trip_id as tid,
		trip.headsign as ths,
		trip.short_name as tsn,
		trip.direction_id as td,
		trip.feed_id as tfid
		FROM trip, route, calendar 
		WHERE trip.uid IN ( 
			SELECT trip.uid
			FROM trip 
			WHERE EXISTS ( 
				SELECT 1 
				FROM stop AS s 
				INNER JOIN stop_time AS st 
				ON s.id = st.stop_id AND s.feed_id = st.feed_id 
				WHERE ST_Within(s.position::geometry, 
							ST_MakeEnvelope($1, $2, $3, $4, 4326))
				AND st.trip_id = trip.trip_id AND trip.feed_id = st.feed_id 
			)
			ORDER BY trip.uid
			LIMIT 50 
		) 
		AND 
		route.feed_id = trip.feed_id AND 
		calendar.feed_id = trip.feed_id AND 
		route.id = trip.route_id AND 
		calendar.service_id = trip.service_id
		ORDER BY trip.uid
	) as trip
	INNER JOIN stop_time ON stop_time.trip_id = tid AND stop_time.feed_id = tfid
	INNER JOIN stop ON stop_time.stop_id = stop.id AND stop_time.feed_id = stop.feed_id
	LEFT JOIN stop as pstop ON stop.id = stop. parent_stop AND stop.feed_id = tfid
	ORDER BY (trip.tuid, stop_sequence)"#;

    let conn = rh.pool.clone().get().unwrap();
    let trips = conn.query(
        query,
        &[&bbox.p1.lng, &bbox.p1.lat, &bbox.p2.lng, &bbox.p2.lat],
    );

    let trips = &trips.unwrap();

    if trips.len() == 0 {
        return Json(ResultArray::<Trip> {
            result: Option::None,
            meta: Meta {
                success: false,
                error: Some(Error {
                    code: 1,
                    message: String::from("Trip not found"),
                }),
            },
        });
    }

    let mut trips_result: Vec<Trip> = Vec::new();
    let mut trips_hm: BTreeMap<Trip, Vec<StopTrip>> = BTreeMap::new();

    let rowuid: String = trips.get(0).get(0);
    println!("{}", rowuid);
    let mut i: i32 = 0;

    for trip_row in trips {
        let uid: String = trip_row.get(0);
        println!("{}: {}", i, uid);
        parse_stop_trip_trip_row(&mut trips_hm, &trip_row);
        i += 1;
    }

    for (k, v) in trips_hm.iter() {
        let mut t = (*k).clone();
        t.stop_sequence = Some(v.clone());
        trips_result.push(t);
    }

    let result = ResultArray::<Trip> {
        result: Some(trips_result),
        meta: Meta {
            success: true,
            error: Option::None,
        },
    };

    Json(result)
}

fn get_stop_trip(trip_uid: String, pool: &Pool<PostgresConnectionManager>) -> Vec<StopTrip> {
    let query =
    r#"SELECT 
    stop.uid, 
    stop.name, 
    ST_Y(position::geometry) as lat,
    ST_X(position::geometry) as lng,
    stop."type", 
    (SELECT stop.uid FROM stop as s WHERE s.id = stop.parent_stop AND s.feed_id = stop.feed_id) as parent_stop, 
    stop_time.stop_sequence, 
    stop_time.drop_off_type,
    stop_time.pickup_type,
    stop_time.arrival_time, 
    stop_time.departure_time 
    FROM stop_time, stop
    WHERE stop.id = stop_time.stop_id
    AND stop_time.trip_id = (SELECT trip.trip_id FROM trip WHERE trip.uid = $1 AND trip.feed_id = stop.feed_id) 
    AND stop.feed_id = stop_time.feed_id 
    ORDER BY stop_sequence ASC"#;

    println!("Query (trip_uid = {}): {}", trip_uid, query);

    let connection = pool.clone().get().unwrap();
    let stop_trips = connection.query(query, &[&trip_uid]);

    let mut stop_trip_result: Vec<StopTrip> = Vec::new();

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

fn parse_stop_trip_trip_row<'a>(trips: &'a mut BTreeMap<Trip, Vec<StopTrip>>, row: &Row) {
    /*
        trip.uid as tuid,
        route.uid,
        calendar.uid,
        trip.trip_id,
        trip.headsign,
        trip.short_name,
        trip.direction_id,
        trip.feed_id
    */
    let mut new_vec: Vec<StopTrip>;
    let mut stop_time_v = Vec::new();
    let mut stop = Stop::new(
        row.get(7),
        row.get(9),
        row.get(10),
        row.get(11),
        row.get(12),
        row.get(13),
    );
    let mut t = Trip::new(
        row.get(0),
        row.get(1),
        row.get(2),
        row.get(3),
        row.get(4),
        row.get(5),
    );

    t.set_feed_id(row.get(6));

    stop.set_id(row.get(8));
    stop.set_feed_id(row.get(6));

    /*
		tuid
		ruid
		cuid
		ths
		tsn
		td
		tfid
		------
	7:	suid
		sid
		sname
		slat
		slng
		st
	13:	uid
		st_at
		st_dt
		st_ss
		st_do
	18:	st_pu
    */
    stop.set_feed_id(row.get(6));

    let drop_off_i: i32 = row.get(17);
    let pickup_i: i32 = row.get(18);

    let drop_off: DropOff = num::FromPrimitive::from_i32(drop_off_i).unwrap();
    let pickup: PickUp = num::FromPrimitive::from_i32(pickup_i).unwrap();

    let arrival_time: NaiveTime = row.get(14);
    let departure_time: NaiveTime = row.get(15);

    let stop_trip = StopTrip {
        stop,
        arrival_time,
        departure_time,
        stop_sequence: row.get(16),
        drop_off,
        pickup,
    };

    if trips.contains_key(&t) {
        let value: Vec<StopTrip> = trips.get(&t).unwrap().to_vec();
        new_vec = value.to_vec();
        new_vec.push(stop_trip);
        trips.insert(t, new_vec);
    } else {
        stop_time_v.push(stop_trip);
        trips.insert(t, stop_time_v);
    }
}

fn parse_stop_trip_row(row: &Row) -> StopTrip {
    let stop = Stop::new(
        row.get(0),
        row.get(1),
        row.get(2),
        row.get(3),
        row.get(4),
        row.get(5),
    );

    let drop_off_i: i32 = row.get(7);
    let pickup_i: i32 = row.get(8);

    let drop_off: DropOff = num::FromPrimitive::from_i32(drop_off_i).unwrap();
    let pickup: PickUp = num::FromPrimitive::from_i32(pickup_i).unwrap();

    let arrival_time: NaiveTime = row.get(9);
    let departure_time: NaiveTime = row.get(10);

    let st = StopTrip {
        stop,
        arrival_time,
        departure_time,
        stop_sequence: row.get(6),
        drop_off,
        pickup,
    };

    st
}
