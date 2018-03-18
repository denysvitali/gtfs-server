use super::model_api::meta::Meta;
use models::time::Time;
use models::trip::Trip;
use models::pickup::PickUp;
use models::dropoff::DropOff;

use super::model_api::result::Result;
use super::model_api::resultarray::ResultArray;

use super::super::RoutesHandler;
use super::super::Json;
use super::super::State;
use super::super::Pool;
use super::super::PostgresConnectionManager;

use postgres::rows::Row;

use num_traits as num;
use num_traits::FromPrimitive;

use super::NaiveTime;

#[get("/times/by-trip/<trip_id>")]
pub fn times_trip(rh: State<RoutesHandler>, trip_id: String) -> Json<ResultArray<Time>>{
    let result = get_times_by_trip(trip_id, &rh.pool);

    let meta = Meta{
        success: true,
        error: None,
    };

    Json(ResultArray::<Time>{
        result,
        meta
    })
}

#[get("/times/by-stop/<stop_id>")]
pub fn times_stop(rh: State<RoutesHandler>, stop_id: String) -> Json<ResultArray<Time>>{
    let result = get_times_by_stop_id(stop_id, &rh.pool);

    let meta = Meta{
        success: true,
        error: None,
    };

    Json(ResultArray::<Time>{
        result,
        meta
    })
}

fn get_times_by_trip(trip_id: String, pool: &Pool<PostgresConnectionManager>) -> Vec<Time>{
    /*
        SELECT * FROM stop_time WHERE
        stop_time.trip_id =
            (SELECT trip.trip_id FROM trip WHERE trip.uid='t-b119d6-lamone-cadempinostazione')
        AND stop_time.feed_id =
            (SELECT trip.feed_id FROM trip WHERE trip.uid='t-b119d6-lamone-cadempinostazione');
    */

    let query = "SELECT  trip_id,\
        arrival_time,\
        departure_time,\
        stop.uid,\
        stop_sequence,\
        pickup_type,\
        drop_off_type,\
        a.feed_id \
        FROM (SELECT * FROM stop_time \
        WHERE \
        stop_time.trip_id = (\
            SELECT trip.trip_id FROM trip WHERE trip.uid=$1\
        ) AND \
        stop_time.feed_id = (\
            SELECT trip.feed_id FROM trip WHERE trip.uid=$1\
        ) \
        ORDER BY stop_sequence) as a INNER JOIN stop ON (a.stop_id=stop.id) WHERE a.feed_id = stop.feed_id";

    let conn = pool.clone().get().unwrap();
    let times = conn.query(
        query, &[&trip_id]
    );

    let mut times_result : Vec<Time> = Vec::new();

    for row in times.expect("Query failed").iter() {
        let time = parse_time_row(&row);
        times_result.push(time);
    }

    times_result
}

fn get_times_by_stop_id(stop_id: String, pool: &Pool<PostgresConnectionManager>) -> Vec<Time>{
    /*
        select stop_time.trip_id as tid FROm stop_time
        WHERE stop_time.stop_id =
            (SELECT  stop.id FROm stop where stop.uid = 's-c27ebe-mannolamonda')
        AND stop_time.feed_id =
            (SELECT stop.feed_id FROM stop WHERE stop.uid='s-c27ebe-mannolamonda')
        GROUP BY trip_id ORDER BY trip_id ASC;
    */

    let query = "SELECT t.uid,\
        arrival_time,\
        departure_time,\
        stop.uid,\
        stop_sequence,\
        pickup_type,\
        drop_off_type,\
        a.feed_id \
        FROM (SELECT * FROM stop_time \
        WHERE \
        stop_time.stop_id = (\
            SELECT stop.id FROM stop WHERE stop.uid=$1\
        ) AND \
        stop_time.feed_id = (\
            SELECT stop.feed_id FROM stop WHERE stop.uid=$1\
        ) \
        ORDER BY arrival_time DESC) as a \
        INNER JOIN stop ON (a.stop_id=stop.id) \
        INNER JOIN trip as t ON (a.trip_id = t.trip_id) \
        WHERE a.feed_id = stop.feed_id AND t.feed_id = stop.feed_id";

    let conn = pool.clone().get().unwrap();
    let times = conn.query(
        query, &[&stop_id]
    );

    let mut times_result : Vec<Time> = Vec::new();

    for row in times.expect("Query failed").iter() {
        let time = parse_time_row(&row);
        times_result.push(time);
    }

    times_result
}

fn parse_time_row(row: &Row) -> Time {

    let pickup_int : i32 = row.get(5);
    let dropoff_int : i32 = row.get(6);

    let arrival : NaiveTime = row.get(1);
    let departure : NaiveTime = row.get(2);

    let mut time = Time::new(
        row.get(0),
        arrival.format("%H:%M:%S").to_string(),
        departure.format("%H:%M:%S").to_string(),
        row.get(3),
        row.get(4),
        num::FromPrimitive::from_i32(pickup_int).unwrap(),
        num::FromPrimitive::from_i32(dropoff_int).unwrap()
    );

    time.set_feed_id(row.get(7));

    time
}