use super::model_api::stopresult::StopResult;
use super::model_api::stopresult::StopDistance;
use super::model_api::stopresult::StopDistanceResult;
use super::model_api::meta::Meta;
use super::model_api::error::Error;

use models::stop::Stop;

use super::super::RoutesHandler;
use super::super::Json;
use super::super::State;
use super::super::Pool;
use super::super::PostgresConnectionManager;

#[get("/stops")]
pub fn stops(rh: State<RoutesHandler>) -> Json<StopResult> {

    let sr = StopResult {
        result: get_stops(&rh.pool),
        meta: Meta{
            success: true,
            error: Error{ code: 0, message: String::new() }
        }
    };
    Json(sr)
}

#[get("/stops/near/<lat>/<lng>/<meters>")]
pub fn stops_near(rh: State<RoutesHandler>, lat: f32, lng: f32, meters: f64) -> Json<StopDistanceResult> {

    let sr = StopDistanceResult {
        result: get_stops_near(&rh.pool, lat, lng, meters),
        meta: Meta{
            success: true,
            error: Error{ code: 0, message: String::new() }
        }
    };
    Json(sr)
}

fn get_stops(pool: &Pool<PostgresConnectionManager>) -> Vec<Stop> {
    let query = "SELECT
        id,
        name,
        type,
        parent_stop,
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
        //let a : String = row.get(2);
        let lat : f64 = row.get(5);
        let lng : f64 = row.get(6);

        let stop = Stop {
            id: row.get(0),
            name: row.get(1),
            lat,
            lng,
            location_type: row.get(2),
            parent_station: row.get(3),
            feed_id: row.get(4)
        };

        stops_result.push(stop);
    }

    stops_result
}

fn get_stops_near(pool: &Pool<PostgresConnectionManager>,
                  lat: f32,
                  lng: f32,
                  meters: f64) -> Vec<StopDistance> {
    let query = "SELECT * FROM (SELECT
        id,
        name,
        type,
        parent_stop,
        feed_id,
        ST_Distance(position, \
        ST_GeomFromText($1)) as distance,
        ST_Y(position::geometry) as lat,
        ST_X(position::geometry) as lng FROM stop) as s1 WHERE \
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
        //let a : String = row.get(2);
        let lat : f64 = row.get(5);
        let lng : f64 = row.get(6);

        let stop = Stop {
            id: row.get(0),
            name: row.get(1),
            lat,
            lng,
            location_type: row.get(2),
            parent_station: row.get(3),
            feed_id: row.get(4)
        };

        let distance = row.get(5);

        let sd = StopDistance {
            stop,
            distance
        };

        stops_result.push(sd);
    }

    stops_result
}