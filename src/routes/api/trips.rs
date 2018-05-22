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
use models::api::paginatedvec::PaginatedVec;
use models::api::pagination::Pagination;
use models::query::Query;
use models::api::search::ascdesc::AscDesc;

use std::str::FromStr;
use models::api::sort::tripsort::TripSort;
use std::cmp::Ordering;

fn trips_query_filter(ts: &TripSearch,
                      query: Query,
                      params: Vec<&ToSql>,
                      pool: &Pool<PostgresConnectionManager>,
                      has_times: bool,
                      has_stop: bool
) -> PaginatedVec<Trip> {

    let mut has_times = has_times;
    let mut has_stop = has_stop;

    let mut query : Query = query.clone();
    let mut trips_result: Vec<Trip> = Vec::new();
    let mut ints : Vec<i64> = Vec::new();
    let mut values: Vec<String> = Vec::new();
    let mut times : Vec<NaiveTime> = Vec::new();
    let mut i = params.len();
    let mut params: Vec<&ToSql> = Vec::from(params);
    let mut addition: String;

    if ts.departure_after.is_some() {
        let departure_after_str = ts.departure_after.as_ref().unwrap();
        let departure_after =
            NaiveTime::parse_from_str(&departure_after_str,
                                      "%H:%M:%S"
            ).unwrap();

        if !has_times {
            query.join_v.push(String::from_str(
                "INNER JOIN stop_time ON stop_time.trip_id = tid AND stop_time.feed_id = tfid"
            ).unwrap());
            has_times = true;
        }

        if !has_stop {
            query.join_v.push(String::from_str(
                "INNER JOIN stop ON stop_time.stop_id = stop.id AND stop_time.feed_id = stop.feed_id"
            ).unwrap());
            has_stop = true;
        }

        i += 1;
        addition = format!(
            "stop_time.departure_time >= ${} ",
            &i
        );

        times.push(departure_after);
        query.where_v.push(addition);
    }

    if ts.arrival_before.is_some() {
        let arrival_before =
            NaiveTime::parse_from_str(ts.arrival_before.as_ref().unwrap(),
                                      "%H:%M:%S"
            ).unwrap();

        if !has_times {
            query.join_v.push(
                String::from_str(
                    "INNER JOIN stop_time ON stop_time.trip_id = tid AND stop_time.feed_id = tfid"
                ).unwrap()
            );
            has_times = true;
        }

        if !has_stop {
            query.join_v.push(
                String::from_str(
                    "INNER JOIN stop ON stop_time.stop_id = stop.id AND stop_time.feed_id = stop.feed_id"
                ).unwrap()
            );
            has_stop = true;
        }

        i += 1;
        addition = format!(
            "stop_time.arrival_time <= ${} ",
            &i
        );

        times.push(arrival_before);
        query.where_v.push(addition);
    }

    for time in &times {
        params.push(time);
    }

    if ts.route.is_some() {
        i+= 1;

        let route_uid : String = ts.route.as_ref().unwrap().to_string();

        addition = format!("ruid = ${}", &i);
        values.push(route_uid);
        query.where_v.push(addition);
    }

    if ts.stops_visited.is_some() {
        let mut where_string = String::new();
        addition = format!(" AND t.uid IN ( ");
        where_string.push_str(&addition);
        let split_stops: Vec<&str> = ts.stops_visited.as_ref().unwrap().split(",").collect();
        let mut first = true;
        for stop in split_stops {
            if first {
                first = !first;
            } else {
                addition = format!(" INTERSECT ");
                where_string.push_str(&addition);
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
            where_string.push_str(&addition);
            values.push(String::from(stop));
            println!("Stop {} ", stop);
        }
        addition = format!(" )");
        where_string.push_str(&addition);
        query.where_v.push(where_string);
    }

    /*if ts.sort_by.is_some() {
        let v_v : TripSort = ts.sort_by.as_ref().unwrap().clone();
        let v : Option<&str> = (match v_v {
            TripSort::ArrivalTime => Some("stop_time.arrival_time"),
            TripSort::DepartureTime => Some("stop_time.departure_time"),
            _ => None
        });

        if v.is_some(){
            if v_v == TripSort::ArrivalTime ||
                v_v == TripSort::DepartureTime {
                // Ignore, we reorder it afterwards!
            } else {
                query.order_v.push(String::from(v.unwrap()));
                // Set the recently added item to the first position:
                query.order_v.rotate_right(1);
            }
        }
    }*/

    if ts.sort_order.as_ref().is_some() {
        query.sort_order = ts.sort_order.as_ref().unwrap().clone();
    }

    let mut offset : i64 = 0;
    let mut limit: i64  = 50;

    if ts.offset.is_some() {
        let c_offset = ts.offset.unwrap();
        if c_offset > 0 {
            offset = c_offset;
        }
    }

    if ts.per_page.is_some() {
        let c_limit = ts.per_page.unwrap();
        if c_limit > 0 && c_limit <= 500 {
            limit = c_limit;
        }
    }

    for value in &values {
        params.push(value);
    }

    addition = format!(" LIMIT ${} OFFSET ${}", &i, &(i + 1));
    query.limit = limit;
    query.offset = offset;

    for value in &ints {
        params.push(value);
    }

    println!("Query: {}", query.format());

    let conn = pool.clone().get().unwrap();
    let trips = conn.query(&query.format(), &params);


    if has_times && has_stop {
        let mut trips_hm: BTreeMap<Trip, Vec<StopTrip>> = BTreeMap::new();

        let mut i: i32 = 0;

        for row in trips.expect("Query failed").iter() {
            let uid: String = row.get(0);
            parse_stop_trip_trip_row(&mut trips_hm, &row);
            i += 1;
        }

        for (k, v) in trips_hm.iter() {
            let mut t = (*k).clone();
            t.stop_sequence = Some(v.clone());
            trips_result.push(t);

        }

        if ts.sort_by.is_some() {
            let v_v = ts.sort_by.as_ref().unwrap();
            if v_v == &TripSort::ArrivalTime || v_v == &TripSort::DepartureTime {
                // Handle sorting of BTreeMap
                match v_v {
                    &TripSort::ArrivalTime => {
                        trips_result.sort_by(|a, b| {
                            if a.stop_sequence.as_ref().is_some() {
                                if b.stop_sequence.as_ref().is_some() {
                                    let at_a = a.stop_sequence.as_ref().unwrap().get(0)
                                        .unwrap().arrival_time;
                                    let at_b = b.stop_sequence.as_ref().unwrap().get(0)
                                        .unwrap().arrival_time;
                                    return at_a.cmp(&at_b);
                                } else {
                                    return Ordering::Less;
                                }
                            } else {
                                return Ordering::Greater;
                            }
                        })
                    },
                    &TripSort::DepartureTime => {
                        trips_result.sort_by(|a, b| {
                            if a.stop_sequence.as_ref().is_some() {
                                if b.stop_sequence.as_ref().is_some() {
                                    let at_a = a.stop_sequence.as_ref().unwrap().get(0)
                                        .unwrap().departure_time;
                                    let at_b = b.stop_sequence.as_ref().unwrap().get(0)
                                        .unwrap().departure_time;
                                    return at_a.cmp(&at_b);
                                } else {
                                    return Ordering::Less;
                                }
                            } else {
                                return Ordering::Greater;
                            }
                        })
                    }
                    _ => {}
                }
            }
        }

    } else {
        for row in trips.expect("Query failed").iter() {
            let mut route = parse_trip_row(&row);
            route.stop_sequence = Option::None;
            trips_result.push(route);
        }
    }

    return PaginatedVec {
        vec: trips_result,
        pag: Some(Pagination{
            limit,
            offset
        })
    };
}

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
            pagination: Option::None
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
    let trips_result: PaginatedVec<Trip> = get_trips_by_query(&query, &rh.pool);

    let rr = ResultArray::<Trip> {
        result: Some(trips_result.vec),
        meta: Meta {
            success: true,
            error: Option::None,
            pagination: trips_result.pag
        },
    };

    Json(rr)
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
            pagination: Option::None
        },
    };

    Json(rr)
}

fn get_trips_by_query(ts: &TripSearch, pool: &Pool<PostgresConnectionManager>) -> PaginatedVec<Trip> {

    let mut query : Query = Query {
        select_v: Vec::new(),
        from_v: Vec::new(),
        where_v: Vec::new(),
        join_v: Vec::new(),
        order_v: Vec::new(),
        limit: 0,
        offset: 0,
        format: String::new(),
        sort_order: AscDesc::ASC,
    };

    query.select_v.push(String::from("t.uid"));
    query.select_v.push(String::from("r.uid"));
    query.select_v.push(String::from("c.uid"));
    query.select_v.push(String::from("trip_id"));
    query.select_v.push(String::from("headsign"));
    query.select_v.push(String::from("t.short_name"));
    query.select_v.push(String::from("direction_id"));
    query.select_v.push(String::from("t.feed_id"));

    query.from_v.push(String::from("trip as t"));
    query.join_v.push(
        String::from(
            "INNER JOIN calendar as c ON c.service_id=t.service_id"
        )
    );

    query.join_v.push(
        String::from(
            "INNER JOIN route as r ON r.id = t.route_id"
        )
    );

    query.where_v.push(
        String::from(
            "c.feed_id = t.feed_id"
        )
    );

    query.where_v.push(
        String::from(
            "r.feed_id = t.feed_id"
        )
    );

    return trips_query_filter(ts, query, vec![], pool, false, false);
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
                pagination: Option::None
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
            pagination: Option::None
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
    // TODO: Pagination
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
                pagination: Option::None
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
            pagination: Option::None
        },
    };

    Json(result)
}

/// `/trips/in/<bbox>`, returns the [Trip](../../../models/trip/struct.Trip.html)s contained
/// in a [Bounding Box](../../../models/struct.BoudingBox.html).
/// Returns a [ResultArray](../../../models/api/result/struct.ResultArray.html)
/// <[Trip](../../../models/trip/struct.Trip.html)>
///
#[get("/trips/in/<bbox>")]
pub fn trips_by_bbox(rh: State<RoutesHandler>, bbox: BoundingBox) -> Json<ResultArray<Trip>> {
    return trips_by_bbox_query(rh, bbox, TripSearch {
        stops_visited: None,
        route: None,
        departure_after: None,
        arrival_before: None,
        offset: None,
        per_page: None,
        sort_by: None,
        sort_order: None,
    });
}

/// `/trips/in/<bbox>?<query>`, returns the [Trip](../../../models/trip/struct.Trip.html)s contained
/// in a [Bounding Box](../../../models/struct.BoudingBox.html), filtered w/ the TripSearch query.
/// Returns a [ResultArray](../../../models/api/result/struct.ResultArray.html)
/// <[Trip](../../../models/trip/struct.Trip.html)>
///
#[get("/trips/in/<bbox>?<ts>")]
pub fn trips_by_bbox_query(rh: State<RoutesHandler>, bbox: BoundingBox, ts: TripSearch)
    -> Json<ResultArray<Trip>> {

    let mut query : Query = Query {
        select_v: Vec::new(),
        from_v: Vec::new(),
        where_v: Vec::new(),
        join_v: Vec::new(),
        order_v: Vec::new(),
        limit: 0,
        offset: 0,
        format: String::new(),
        sort_order: AscDesc::ASC,
    };

    query.format = String::from(r#"SELECT
        {0}
        FROM {1}
        {2}
        WHERE {3}
        {4}
    "#);

    // We didn't include {5} because we limit the query in from_v

    query.select_v.push(String::from_str("tuid").unwrap());
    query.select_v.push(String::from_str("ruid").unwrap());
    query.select_v.push(String::from_str("cuid").unwrap());
    query.select_v.push(String::from_str("ths").unwrap());
    query.select_v.push(String::from_str("tsn").unwrap());
    query.select_v.push(String::from_str("td").unwrap());
    query.select_v.push(String::from_str("tfid").unwrap());
    query.select_v.push(String::from_str("stop.uid as suid").unwrap());
    query.select_v.push(String::from_str("stop.id as sid").unwrap());
    query.select_v.push(String::from_str("stop.\"name\" as sname").unwrap());
    query.select_v.push(String::from_str("ST_Y(stop.position::geometry) as slat").unwrap());
    query.select_v.push(String::from_str("ST_X(stop.position::geometry) as slng").unwrap());
    query.select_v.push(String::from_str("stop.\"type\" as st").unwrap());
    query.select_v.push(String::from_str("pstop.uid").unwrap());
    query.select_v.push(String::from_str("stop_time.arrival_time as st_at").unwrap());
    query.select_v.push(String::from_str("stop_time.departure_time as st_dt").unwrap());
    query.select_v.push(String::from_str("stop_time.stop_sequence as st_ss").unwrap());
    query.select_v.push(String::from_str("stop_time.drop_off_type as st_do").unwrap());
    query.select_v.push(String::from_str("stop_time.pickup_type as st_pu").unwrap());

    query.from_v.push(String::from_str(r#"(SELECT DISTINCT
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
			SELECT DISTINCT trip.uid
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
			{0}
		)
		AND
		route.feed_id = trip.feed_id AND
		calendar.feed_id = trip.feed_id AND
		route.id = trip.route_id AND
		calendar.service_id = trip.service_id
		GROUP BY tuid, ruid, cuid, tid, ths, tsn, td, tfid
		ORDER BY trip.uid
	) as trip"#).unwrap());

    query.join_v.push(
        String::from_str(
            "INNER JOIN stop_time ON stop_time.trip_id = tid AND stop_time.feed_id = tfid"
        ).unwrap()
    );
    query.join_v.push(
        String::from_str(
            "INNER JOIN stop ON stop_time.stop_id = stop.id AND stop_time.feed_id = stop.feed_id"
        ).unwrap()
    );
    query.join_v.push(
        String::from_str(
            "LEFT JOIN stop as pstop ON stop.id = stop. parent_stop AND stop.feed_id = tfid"
        ).unwrap()
    );

    query.order_v.push(String::from_str("tuid").unwrap());
    query.order_v.push(String::from_str("stop_time.stop_sequence").unwrap());

    let params: Vec<&ToSql> =
        vec![&bbox.p1.lng, &bbox.p1.lat, &bbox.p2.lng, &bbox.p2.lat];

    let paginated_result = trips_query_filter(&ts,
                                              query,
                                              params,
                                              &rh.pool,
                                              true,
                                              true);


    let result = ResultArray::<Trip> {
        result: Some(paginated_result.vec),
        meta: Meta {
            success: true,
            error: Option::None,
            pagination: paginated_result.pag
        },
    };

    return Json(
        result
    );
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
