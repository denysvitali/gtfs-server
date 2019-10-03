//! `/stops` related routes

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

use models::coordinate::Coordinate;
use postgres::NoTls;

static GMAPS_API_KEY: &'static str = "AIzaSyAvtzzsAPAlOrK8JbGfXfHMt18MbqCqrj4";

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
        meta: Meta {
            success: true,
            error: Option::None,
            pagination: Option::None,
        },
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
                    message: String::from("Invalid stop id"),
                }),
                pagination: Option::None,
            },
        });
    }

    let sr = Result::<Stop> {
        result: stop,
        meta: Meta {
            success: true,
            error: Option::None,
            pagination: Option::None,
        },
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
        meta: Meta {
            success: true,
            error: Option::None,
            pagination: Option::None,
        },
    };
    Json(sr)
}

/// `/stops/near/<lat>/<lng>`  
/// Gets an array of [StopDistance](../../../models/api/stopdistance/struct.StopDistance.html)s,
/// within 100.0 meters from `<lat>`,`<lng>` - nearest first.  
/// Returns a [ResultArray](../../../models/api/resultarray/struct.ResultArray.html)
/// <[StopDistance](../../../models/api/stopdistance/struct.StopDistance.html)>
#[get("/stops/near/<lat>/<lng>")]
pub fn stops_near_default(
    rh: State<RoutesHandler>,
    lat: f32,
    lng: f32,
) -> Json<ResultArray<StopDistance>> {
    let sr = ResultArray::<StopDistance> {
        result: Some(get_stops_near(&rh.pool, lat, lng, 100.0)),
        meta: Meta {
            success: true,
            error: Option::None,
            pagination: Option::None,
        },
    };
    Json(sr)
}

/// `/stops/near/<lat>/<lng>/<meters>`  
/// Gets an array of [StopDistance](../../../models/api/stopdistance/struct.StopDistance.html)s,
/// within `<meters>` meters from `<lat>`,`<lng>`
/// nearest first, of Stops near the provided coordinate.  
/// Returns a [ResultArray](../../../models/api/resultarray/struct.ResultArray.html)
/// <[StopDistance](../../../models/api/stopdistance/struct.StopDistance.html)>
#[get("/stops/near/<lat>/<lng>/<meters>")]
pub fn stops_near(
    rh: State<RoutesHandler>,
    lat: f32,
    lng: f32,
    meters: f64,
) -> Json<ResultArray<StopDistance>> {
    let sr = ResultArray::<StopDistance> {
        result: Some(get_stops_near(&rh.pool, lat, lng, meters)),
        meta: Meta {
            success: true,
            error: Option::None,
            pagination: Option::None,
        },
    };
    Json(sr)
}

/// `/stops/in/<bbox>`  
/// Gets an array of [Stop](../../../models/api/struct.Stop.html)s,
/// inside a [BoudingBox](../../models/struct.boudingbox.html) defined by two points (P1 and P2).
///
/// Returns a [ResultArray](../../../models/api/resultarray/struct.ResultArray.html)
/// <[Stop](../../../models/api/sruct.Stop.html)>
#[get("/stops/in/<bbox>")]
pub fn stops_in_bbox(rh: State<RoutesHandler>, bbox: BoundingBox) -> Json<ResultArray<Stop>> {
    let sr = ResultArray::<Stop> {
        result: Some(get_stops_in_bbox(&rh.pool, bbox)),
        meta: Meta {
            success: true,
            error: Option::None,
            pagination: Option::None,
        },
    };
    Json(sr)
}

/// `/stops/in/<bbox>/<meters>`  
/// Gets an array of [Stop](../../../models/api/struct.Stop.html)s,
/// inside a [Bouding Box](../../models/struct.boudingbox.html) defined by two circles of a radius `<meters>` meters
/// with centers in P1 and P2.
///
/// Returns a [ResultArray](../../../models/api/resultarray/struct.ResultArray.html)
/// <[Stop](../../../models/api/sruct.Stop.html)>
#[get("/stops/in/<bbox>/<radius>")]
pub fn stops_in_bbox_radius(
    rh: State<RoutesHandler>,
    bbox: BoundingBox,
    radius: f64,
) -> Json<ResultArray<Stop>> {
    let bbox = get_bbox_from_points(bbox.p1, bbox.p2, radius, radius);
    let sr = ResultArray::<Stop> {
        result: Some(get_stops_in_bbox(&rh.pool, bbox)),
        meta: Meta {
            success: true,
            error: Option::None,
            pagination: Option::None,
        },
    };
    Json(sr)
}

#[get("/stops/test/<lat>/<lng>/<lat2>/<lng2>/<rad1>/<rad2>")]
pub fn stops_latlng_test(
    lat: f64,
    lng: f64,
    lat2: f64,
    lng2: f64,
    rad1: f64,
    rad2: f64,
) -> content::Html<String> {
    content::Html(get_stops_by_coordinates(
        Coordinate { lat, lng },
        Coordinate {
            lat: lat2,
            lng: lng2,
        },
        rad1,
        rad2,
        14,
    ))
}

#[get("/stops/test/<lat>/<lng>/<lat2>/<lng2>/<rad1>/<rad2>/<zoom>")]
pub fn stops_latlng_test_zoom(
    lat: f64,
    lng: f64,
    lat2: f64,
    lng2: f64,
    rad1: f64,
    rad2: f64,
    zoom: i32,
) -> content::Html<String> {
    content::Html(get_stops_by_coordinates(
        Coordinate { lat, lng },
        Coordinate {
            lat: lat2,
            lng: lng2,
        },
        rad1,
        rad2,
        zoom,
    ))
}

fn parse_stop_row(row: &Row) -> Stop {
    let lat: f64 = row.get(6);
    let lng: f64 = row.get(7);

    let uid = row.get(0);
    let id = row.get(1);
    let name = row.get(2);
    let location_type: Option<i32> = row.get(3);
    let parent_station: Option<String> = row.get(4);
    let feed_id = row.get(5);

    let mut stop = Stop::new(uid, name, lat, lng, location_type, parent_station);

    &stop.set_id(id);
    &stop.set_feed_id(feed_id);

    stop
}

fn get_stop_by_id(stop_id: String, pool: &Pool<PostgresConnectionManager<NoTls>>) -> Option<Stop> {
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

    let mut conn = pool.clone().get().unwrap();
    let stops = conn.query(query, &[&stop_id]);

    let stops = &stops.expect("Query failed");

    if stops.len() != 1 {
        return Option::None;
    }

    let stop = parse_stop_row(&stops.get(0).unwrap());
    Some(stop)
}

fn get_stops(pool: &Pool<PostgresConnectionManager<NoTls>>) -> Vec<Stop> {
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

    let mut conn = pool.clone().get().unwrap();
    let stops = conn.query(query, &[]);

    let mut stops_result: Vec<Stop> = Vec::new();

    for row in stops.expect("Query failed").iter() {
        let stop = parse_stop_row(&row);
        stops_result.push(stop);
    }

    stops_result
}

fn get_stops_near(
    pool: &Pool<PostgresConnectionManager<NoTls>>,
    lat: f32,
    lng: f32,
    meters: f64,
) -> Vec<StopDistance> {
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
    let mut conn = pool.clone().get().unwrap();
    let stops = conn.query(query, &[&format!("POINT({:.5} {:.5})", lng, lat), &meters]);

    let mut stops_result: Vec<StopDistance> = Vec::new();

    for row in stops.expect("Query failed").iter() {
        let stop = parse_stop_row(&row);
        let distance = row.get(8);

        let sd = StopDistance { stop, distance };

        stops_result.push(sd);
    }

    stops_result
}

fn get_stops_in_bbox(
    pool: &Pool<PostgresConnectionManager<NoTls>>,
    bbox: BoundingBox,
) -> Vec<Stop> {
    let query = "SELECT \
        uid,
        id,
        name,
        type,
        (SELECT s.uid FROM stop as s WHERE s.id = stop.parent_stop AND s.feed_id = stop.feed_id) as parent_stop,
        feed_id,
        ST_Y(position::geometry) as lat,
        ST_X(position::geometry) as lng
        FROM stop \
        WHERE ST_Contains( \
            ST_MakeEnvelope( \
                $2, \
                $1, \
                $4, \
                $3, \
                4326), \
            position::geometry \
        ) \
        LIMIT 5000;";
    let p1 = bbox.p1;
    let p2 = bbox.p2;
    //println!(format!("{}", query));

    let mut conn = pool.clone().get().unwrap();
    let stops = conn.query(query, &[&p1.lat, &p1.lng, &p2.lat, &p2.lng]);

    let mut stops_result: Vec<Stop> = Vec::new();

    for row in stops.expect("Query failed").iter() {
        let stop = parse_stop_row(&row);
        stops_result.push(stop);
    }

    stops_result
}

fn get_stops_by_trip(trip_id: String, pool: &Pool<PostgresConnectionManager<NoTls>>) -> Vec<Stop> {
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

    let mut conn = pool.clone().get().unwrap();
    let stops = conn.query(query, &[&trip_id]);

    let mut stops_result: Vec<Stop> = Vec::new();

    for row in stops.expect("Query failed").iter() {
        let stop = parse_stop_row(&row);

        stops_result.push(stop);
    }

    stops_result
}

fn deg2rad(deg: f64) -> f64 {
    (deg / 360.0) * 2.0 * f64::consts::PI
}

fn deg2meter_lng(lat: f64) -> f64 {
    // X: E - W
    111412.84 * deg2rad(lat).cos() - 93.5 * deg2rad(3.0 * lat).cos()
        + 0.118 * deg2rad(5.0 * lat).cos()
}

fn deg2meter_lat(lat: f64) -> f64 {
    // Y: N - S
    111132.92 - 559.82 * deg2rad(2.0 * lat).cos() + 1.175 * deg2rad(4.0 * lat).cos()
        - 0.0023 * deg2rad(6.0 * lat).cos()
}

fn get_stops_by_coordinates(c1: Coordinate, c2: Coordinate, r1: f64, r2: f64, zoom: i32) -> String {
    let result: Vec<Stop> = Vec::new();

    println!("Lat Sec meter at 30°: {}", deg2meter_lat(0.0));
    let bbox = get_bbox_from_points(c1, c2, r1, r2);
    let p1 = bbox.p1;
    let p2 = bbox.p2;

    println!("P1: {}, {}", p1.lat, p1.lng);
    println!("P2: {}, {}", p2.lat, p2.lng);
    format!(r#"
    <!DOCTYPE html>
    <html>
    <head>
    <title>Stop Test</title>
    </head>
    <body>
    <img src="https://maps.googleapis.com/maps/api/staticmap?center={0},{1}&zoom={5}&scale=1&size=600x300&maptype=roadmap&format=png&visual_refresh=true&markers=size:mid%7Ccolor:0x009fff%7Clabel:A%7C{0},{1}&key={4}">
    <img src="https://maps.googleapis.com/maps/api/staticmap?center={2},{3}&zoom={5}&scale=1&size=600x300&maptype=roadmap&format=png&visual_refresh=true&markers=size:mid%7Ccolor:0x009fff%7Clabel:B%7C{2},{3}&key={4}">"
    <table>
        <tr>
            <th>P</th>
            <th>Lat</th>
            <th>Lng</th>
        </tr>

        <tr>
            <td>1</td>
            <td>{0}</td>
            <td>{1}</td>
        </tr>

        <tr>
            <td>2</td>
            <td>{2}</td>
            <td>{3}</td>
        </tr>
    </table>
    </body>
    </html>
    "#,
    p1.lat, p1.lng,
    p2.lat, p2.lng,
    GMAPS_API_KEY,
    zoom)
}

fn get_bbox_from_points(c1: Coordinate, c2: Coordinate, r1: f64, r2: f64) -> BoundingBox {
    // LAT = North to South (Y)
    // LNG = East to West   (X)

    let mut p1: Coordinate = Coordinate { lat: 0.0, lng: 0.0 };

    let mut p2: Coordinate = Coordinate { lat: 0.0, lng: 0.0 };

    if c1.lng < c2.lng {
        // Blue
        p1.lng = c1.lng - r1 * 1.0 / deg2meter_lng(c1.lat);
        p1.lat = c1.lat + r1 * 1.0 / deg2meter_lat(c1.lat);

        p2.lng = c2.lng + r2 * 1.0 / deg2meter_lng(c2.lat);
        p2.lat = c2.lat + r2 * 1.0 / deg2meter_lat(c2.lat);

        println!("C1");
    } else if c1.lng > c2.lng {
        // Orange
        p1.lng = c2.lng - r2 * 1.0 / deg2meter_lng(c2.lat);
        p1.lat = c2.lat + r2 * 1.0 / deg2meter_lat(c2.lat);

        p2.lng = c1.lng + r1 * 1.0 / deg2meter_lng(c1.lat);
        p2.lat = c1.lat - r1 * 1.0 / deg2meter_lat(c1.lat);
        println!("C2");
    } else {
        if c1.lng > c2.lng {
            // Orange
            p1.lng = c2.lng + r2 * 1.0 / deg2meter_lng(c2.lat);
            p1.lat = c2.lat + r2 * 1.0 / deg2meter_lat(c2.lat);

            p2.lng = c1.lng - r1 * 1.0 / deg2meter_lng(c1.lat);
            p2.lat = c1.lat - r1 * 1.0 / deg2meter_lat(c1.lat);
            println!("C3");
        } else {
            // Blue
            p1.lng = c1.lng + r1 * 1.0 / deg2meter_lng(c1.lat);
            p1.lat = c1.lat - r1 * 1.0 / deg2meter_lat(c1.lat);

            p2.lng = c2.lng - r2 * 1.0 / deg2meter_lng(c2.lat);
            p2.lat = c2.lat - r2 * 1.0 / deg2meter_lat(c2.lat);
            println!("C4");
        }
    }

    BoundingBox { p1, p2 }
}
