//! `/times` related routes

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
use super::NaiveDate;

use postgres::types::ToSql;

use super::model_api::search::time::TimeSearch;
use super::model_api::search::time::TimeSort;
use super::model_api::search::ascdesc::AscDesc;

#[get("/times?<time_search>")]
pub fn times_query(rh: State<RoutesHandler>, time_search: TimeSearch) -> Json<ResultArray<Time>>{
    let result = get_times_by_query(&time_search, &rh.pool);

    let meta = Meta{
        success: true,
        error: None,
    };

    Json(ResultArray::<Time>{
        result: Some(result),
        meta
    })
}

/// `/times/by-trip/<trip_uid>`  
/// Gets the [Time](../../../models/time/struct.Time.html)s associated
/// to the specified [Trip](../../../models/trip/struct.Trip.html) UID, parametrized as `<trip_id>`.  
/// Returns a [ResultArray](../../../models/api/resultarray/struct.ResultArray.html)
/// <[Time](../../../models/time/struct.Time.html)>
#[get("/times/by-trip/<trip_uid>")]
pub fn times_by_trip(rh: State<RoutesHandler>, trip_uid: String) -> Json<ResultArray<Time>>{
    let result = get_times_by_trip(trip_uid, &rh.pool);

    let meta = Meta{
        success: true,
        error: None,
    };

    Json(ResultArray::<Time>{
        result: Some(result),
        meta
    })
}

/// `/times/by-stop/<stop_id>?<time_search>`  
/// Gets the [Time](../../../models/time/struct.Time.html)s associated
/// to the specified [Stop](../../../models/stop/struct.Stop.html) UID, parametrized as `<stop_id>`.  
/// The results can be filtered with `<time_search>` parameters (check [TimeSearch](../../../models/api/search/time/struct.TimeSearch.html))  
/// Returns a [ResultArray](../../../models/api/resultarray/struct.ResultArray.html)
/// <[Time](../../../models/time/struct.Time.html)>
/// 
/// ## Query Parameters
/// ### date
/// A date on which the time is valid.
/// This field only validates the `start_date` and `end_date` fields,
/// this means that you'll still need to check if the requested trip is available in
/// the `date` weekday.
/// 
/// ### service_id
/// Filter by this service_id
/// 
/// ### monday, tuesday, ..., sunday
/// When they're set (to either true or false) a filter will be applied on these week days service conditions
///
/// ### at_a
/// Arrival time is after ... (00:00:00)
/// 
/// ### at_b
/// Arrival time is before ... (00:00:00)
/// 
/// ### dt_a
/// Departure time is after ... (00:00:00)
/// 
/// ### dt_b
/// Departure time is before ... (00:00:00)
/// 
/// ### trip_id
/// Filter by trip_id (for example `t-32c94c-castagnolacapolinea`)
/// 
/// ### pickup_type
/// Only show Time Stops with this pickup condition.  
/// For example: `RegularlyScheduled`
/// 
/// ### drop_off_type
/// Only show Time Stops with this drop-off condition.  
/// For example: `RegularlyScheduled` 
/// 
/// ### stop_sequence
/// Only show Time Stops with this stop_sequence.  
/// For example: `1`
/// 
/// ### sort_by
/// Sort by ...  
/// Possible values are available in the [TimeSort](../../../models/api/search/time/enum.TimeSort.html) enum.  
/// For example: `arrival_time`
///   
/// ### Sort order
/// Ascending (`asc`) or Descending (`desc`)
/// These sorting order conditions will only affect the query if `sort_by` is set.
/// 
/// ## Example
/// Transports that reach `s-bdf67e-luganostazione` (Lugano, Stazione) on a Sunday, between 2PM and 3 PM, sorted by arrival time in Ascending order:  
/// ### Request
/// `/times/by-stop/s-bdf67e-luganostazione?sunday=true&at_a=14:00:00&at_b=15:00:00&sort_by=arrival_time&sort_order=asc`
/// ### Response
/**
    ```json
    {
      "result": [
        {
          "trip_id": "t-32c94c-castagnolacapolinea",
          "arrival_time": "14:07:00",
          "departure_time": "14:07:00",
          "stop_id": "s-bdf67e-luganostazione",
          "stop_sequence": 9,
          "pickup_type": "RegularlyScheduled",
          "drop_off_type": "RegularlyScheduled",
          "service_days": [
            false,
            false,
            false,
            false,
            false,
            false,
            true
          ],
          "service_uid": "se-3e10af-ta-b0013",
          "start_date": "2017-12-10",
          "end_date": "2018-12-08"
        },
        {
          "trip_id": "t-3f5dd9-canobbioganna",
          "arrival_time": "14:07:00",
          "departure_time": "14:10:00",
          "stop_id": "s-bdf67e-luganostazione",
          "stop_sequence": 10,
          "pickup_type": "RegularlyScheduled",
          "drop_off_type": "RegularlyScheduled",
          "service_days": [
            false,
            false,
            false,
            false,
            false,
            true,
            true
          ],
          "service_uid": "se-f80c67-ta-b001r",
          "start_date": "2017-12-10",
          "end_date": "2018-12-08"
        },
        (...)
      ],
      "meta": {
        "success": true
      }
    }
    ```
**/
/// 
#[get("/times/by-stop/<stop_id>?<time_search>")]
pub fn times_stop_query(rh: State<RoutesHandler>, stop_id: String, mut time_search: TimeSearch) -> Json<ResultArray<Time>>{
    time_search.stop = Some(stop_id);
    let result = get_times_by_query(&time_search, &rh.pool);

    let meta = Meta{
        success: true,
        error: None,
    };

    Json(ResultArray::<Time>{
        result: Some(result),
        meta
    })
}

/// `/times/by-stop/<stop_id>`  
/// Gets the [Time](../../../models/time/struct.Time.html)s associated
/// to the specified [Stop](../../../models/stop/struct.Stop.html) UID, parametrized as `<stop_id>`.  
/// Returns a [ResultArray](../../../models/api/resultarray/struct.ResultArray.html)
/// <[Time](../../../models/time/struct.Time.html)>
#[get("/times/by-stop/<stop_id>")]
pub fn times_stop(rh: State<RoutesHandler>, stop_id: String) -> Json<ResultArray<Time>>{
    let result = get_times_by_stop_id(stop_id, &rh.pool);

    let meta = Meta{
        success: true,
        error: None,
    };

    Json(ResultArray::<Time>{
        result: Some(result),
        meta
    })
}

fn get_times_by_query<'a>(time_search: &TimeSearch, pool: &Pool<PostgresConnectionManager>) -> Vec<Time>{

    let mut query = String::from("SELECT t.uid,
        arrival_time,
        departure_time,
        stop.uid,
        stop_sequence,
        pickup_type,
        drop_off_type,
        r.uid,
        c.uid,
        c.monday,
        c.tuesday,
        c.wednesday,
        c.thursday,
        c.friday,
        c.saturday,
        c.sunday,
        c.start_date,
        c.end_date,
        a.feed_id
        FROM stop_time as a
        INNER JOIN stop ON (a.stop_id=stop.id)
        INNER JOIN trip as t ON (a.trip_id = t.trip_id)
        INNER JOIN route as r ON (t.route_id = r.id)
        INNER JOIN calendar as c ON (t.service_id = c.service_id)
        WHERE
        a.feed_id = stop.feed_id AND 
        t.feed_id = stop.feed_id AND 
        c.feed_id = t.feed_id AND
        r.feed_id = c.feed_id AND
        r.id = t.route_id");

    /*
        (SELECT * FROM stop_time
        WHERE
        stop_time.stop_id = (
            SELECT stop.id FROM stop WHERE stop.uid=$1
        ) AND
        stop_time.feed_id = (
            SELECT stop.feed_id FROM stop WHERE stop.uid=$1
        )
        ) as a
    */

    let mut dates : Vec<NaiveDate> = Vec::new();
    let mut times : Vec<NaiveTime> = Vec::new();
    let mut i32_values : Vec<i32> = Vec::new();
    let mut values : Vec<&bool> = Vec::new();
    let mut string_values : Vec<&String> = Vec::new();
    let mut params: Vec<&ToSql> = Vec::new();
    let mut i = 0;

    let mut addition : String;

    string_values = Vec::new();

    if time_search.stop.is_some() {
        string_values.push(time_search.stop.as_ref().unwrap());
        i+= 1;
        addition = format!(" AND a.stop_id = (SELECT stop.id FROM stop WHERE stop.uid = ${} AND stop.feed_id = a.feed_id LIMIT 1) ", &i);
        query.push_str(&addition);
    }

    for &val in string_values.iter() {
        params.push(val);
    }

    string_values = Vec::new();

    

    if time_search.monday.is_some() {
        values.push(time_search.monday.as_ref().unwrap());
        i+= 1;
        addition = format!(" AND monday = ${}", &i);
        query.push_str(&addition);
    }

    if time_search.tuesday.is_some() {
        values.push(time_search.tuesday.as_ref().unwrap());
        i+= 1;
        addition = format!(" AND tuesday = ${}", &i);
        query.push_str(&addition);
    }

    if time_search.wednesday.is_some() {
        values.push(time_search.wednesday.as_ref().unwrap());
        i+= 1;
        addition = format!(" AND wednesday = ${}", &i);
        query.push_str(&addition);
    }

    if time_search.thursday.is_some() {
        values.push(time_search.thursday.as_ref().unwrap());
        i+= 1;
        addition = format!(" AND thursday = ${}", &i);
        query.push_str(&addition);
    }

    if time_search.friday.is_some() {
        values.push(time_search.friday.as_ref().unwrap());
        i+= 1;
        addition = format!(" AND friday = ${}", &i);
        query.push_str(&addition);
    }

    if time_search.saturday.is_some() {
        values.push(time_search.saturday.as_ref().unwrap());
        i+= 1;
        addition = format!(" AND saturday = ${}", &i);
        query.push_str(&addition);
    }

    if time_search.sunday.is_some() {
        values.push(time_search.sunday.as_ref().unwrap());
        i+= 1;
        addition = format!(" AND sunday = ${}", &i);
        query.push_str(&addition);
    }

    for val in &values {
        params.push(val);
    }

    if time_search.date.is_some() {
        string_values.push(
            time_search.date
                .as_ref()
                .unwrap()
        );
        i+= 1;
        addition = format!(" AND start_date <= ${0} AND end_date >= ${0}", &i);
        query.push_str(&addition);
    }

    for &val in string_values.iter() {
        &dates.push(val.parse::<NaiveDate>().unwrap());
    }

    for val in &dates {
        params.push(val);
    }

    i32_values = Vec::new();

    if time_search.stop_sequence.is_some() {
        i32_values.push(time_search.stop_sequence.unwrap());
        i+= 1;
        addition = format!(" AND stop_sequence = ${}", &i);
        query.push_str(&addition);
    }

    if time_search.drop_off_type.is_some() {
        let drop_off : DropOff = DropOff::from_string(time_search.drop_off_type.as_ref().unwrap());

        i32_values.push(
            num::ToPrimitive::to_i32(&drop_off).unwrap()
        );
        i+= 1;
        addition = format!(" AND a.drop_off_type = ${}", &i);
        query.push_str(&addition);
    }

    if time_search.pickup_type.is_some() {
        let pickup : PickUp = PickUp::from_string(time_search.pickup_type.as_ref().unwrap());

        i32_values.push(
            num::ToPrimitive::to_i32(&pickup).unwrap()
        );
        i+= 1;
        addition = format!(" AND a.pickup_type = ${}", &i);
        query.push_str(&addition);
    }

    for val in &i32_values {
        params.push(val);
    }


    string_values = Vec::new();

    if time_search.at_a.is_some() {

        string_values.push(
            time_search.at_a
                .as_ref()
                .unwrap()
        );

        i+= 1;
        addition = format!(" AND arrival_time >= ${}", &i);
        query.push_str(&addition);
    }

    if time_search.at_b.is_some() {
        string_values.push(
            time_search.at_b
                .as_ref()
                .unwrap()
        );

        i+= 1;
        addition = format!(" AND arrival_time <= ${}", &i);
        query.push_str(&addition);
    }

    for &val in string_values.iter() {
        &times.push(val.parse::<NaiveTime>().unwrap());
    }

    for val in &times {
        params.push(val);
    }
    
    string_values = Vec::new();
    
    if time_search.trip_id.is_some() {
        string_values.push(
            time_search.trip_id
                .as_ref()
                .unwrap()
        );
        
        i+= 1;
        addition = format!(" AND t.uid = ${}", &i);
        query.push_str(&addition);
    }

    if time_search.route.is_some() {
        string_values.push(
            time_search.route
                .as_ref()
                .unwrap()
        );
        
        i+= 1;
        addition = format!(" AND r.uid = ${}", &i);
        query.push_str(&addition);
    }

    for &val in string_values.iter() {
        params.push(val);
    }

    if time_search.sort_by.is_some() {
        let mut sort: AscDesc = AscDesc::ASC;
        if time_search.sort_order.as_ref().is_some() {
            let res = time_search.sort_order.as_ref().unwrap();
            sort = match res.to_lowercase().as_str() {
                "asc" => AscDesc::ASC,
                "desc" => AscDesc::DESC,
                _ => AscDesc::ASC
            };
        }
        
        let sort_by = match time_search.sort_by.as_ref().unwrap().as_str() {
            "arrival_time" => TimeSort::arrival_time,
            "departure_time" => TimeSort::departure_time,
            "stop_sequence" => TimeSort::stop_sequence,
            &_ => TimeSort::arrival_time
        };
        
        addition = format!(" ORDER BY {} {}", sort_by.as_str(), sort.as_str());
        query.push_str(&addition);
    }

    query.push_str(" LIMIT 50");

    println!("{}", query);

    /*let query = "SELECT  trip_id,\
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
        ORDER BY stop_sequence) as a INNER JOIN stop ON (a.stop_id=stop.id) WHERE a.feed_id = stop.feed_id";*/

    let conn = pool.clone().get().unwrap();
    let times = conn.query(
        &query, &params.as_slice()
    );

    println!("{:?}", params.as_slice());

    let mut times_result : Vec<Time> = Vec::new();

    for row in times.expect("Query failed").iter() {
        let time = parse_time_row(&row);
        times_result.push(time);
    }

    times_result
}

pub fn get_times_by_trip(trip_uid: String, pool: &Pool<PostgresConnectionManager>) -> Vec<Time>{
    let mut ts : TimeSearch = Default::default();
    ts.trip = Some(trip_uid);
    get_times_by_query(&ts, pool)
}

fn get_times_by_stop_id(stop_uid: String, pool: &Pool<PostgresConnectionManager>) -> Vec<Time>{
    let mut ts : TimeSearch = Default::default();
    ts.stop = Some(stop_uid);
    get_times_by_query(&ts, pool)
}

fn parse_time_row(row: &Row) -> Time {

    let pickup_int : i32 = row.get(5);
    let dropoff_int : i32 = row.get(6);

    let arrival : NaiveTime = row.get(1);
    let departure : NaiveTime = row.get(2);

    let start_date : NaiveDate = row.get(16);
    let end_date: NaiveDate = row.get(17);

    let mut time = Time {
        trip_id: row.get(0),
        arrival_time: arrival.format("%H:%M:%S").to_string(),
        departure_time: departure.format("%H:%M:%S").to_string(),
        stop_id: row.get(3),
        stop_sequence: row.get(4),
        pickup_type: num::FromPrimitive::from_i32(pickup_int).unwrap(),
        drop_off_type: num::FromPrimitive::from_i32(dropoff_int).unwrap(),
        route_id: row.get(7),
        service_days: vec![row.get(9), row.get(10), row.get(11), row.get(12),
                           row.get(13), row.get(14), row.get(15)],
        service_uid: row.get(8),
        start_date,
        end_date,
        feed_id: row.get(18)
    };

    time
}
