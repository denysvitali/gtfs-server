//! `/routes` related routes

use super::model_api::meta::Meta;

use models::route::Route;
use super::model_api::result::Result;
use super::model_api::resultarray::ResultArray;
use super::model_api::error::Error;

use super::agency;

use super::super::RoutesHandler;
use super::super::Json;
use super::super::State;
use super::super::Pool;
use super::super::PostgresConnectionManager;

use postgres::rows::Row;
use models::api::search::route::RouteSearch;

/// `/routes`  
/// Returns a [ResultArray](../../../models/api/resultarray/struct.ResultArray.html)
/// <[Route](../../../models/route/struct.Route.html)>
#[get("/routes")]
pub fn routes(rh: State<RoutesHandler>) -> Json<ResultArray<Route>>{
    let query = "SELECT * FROM route LIMIT 50";
    let conn = rh.pool.clone().get().unwrap();
    let routes = conn.query(
        query,
        &[]
    );
    let mut routes_result: Vec<Route> = Vec::new();

    for row in routes.expect("Query failed").iter() {
        let route = parse_route_row(&row, &rh);
        routes_result.push(route);
    }

    Json(ResultArray{
        result: Some(routes_result),
        meta: Meta {
            success: true,
            error: Option::None
        }
    })
}

/// `/routes?query`
/// Returns a [ResultArray](../../../models/api/resultarray/struct.ResultArray.html)
/// <[Route](../../../models/route/struct.Route.html)>

// TODO: Implement Route Search by RouteSearch query

#[get("/routes?<route_search>")]
pub fn routes_by_query(rh: State<RoutesHandler>, route_search: RouteSearch) -> Json<ResultArray<Route>>{

    let result : Vec<Route> = get_routes_by_query(&route_search, &rh.pool);

    let query = "SELECT * FROM route LIMIT 50";
    let conn = rh.pool.clone().get().unwrap();
    let routes = conn.query(
        query,
        &[]
    );
    let mut routes_result: Vec<Route> = Vec::new();

    for row in routes.expect("Query failed").iter() {
        let route = parse_route_row(&row, &rh);
        routes_result.push(route);
    }

    Json(ResultArray{
        result: Some(routes_result),
        meta: Meta {
            success: true,
            error: Option::None
        }
    })
}

fn get_routes_by_query(route_search: &RouteSearch, pool: &Pool<PostgresConnectionManager>) -> Vec<Route> {
    let mut query = String::from("SELECT * FROM route LIMIT 50");
    let mut addition : String;
    let mut values : Vec<&str> = Vec::new();
    let mut i = 1;
    let mut result : Vec<Route> = Vec::new();

    if route_search.stops_visited.is_some() {
        values.push(route_search.stops_visited.as_ref().unwrap());
        i+= 1;
        addition = format!(" AND ");
    }

    result
}

/// `/routes/<route_uid>`  
/// Gets the specified [Route](../../../models/route/struct.Route.html) by its UID,
/// parametrized as `<route_uid>`.  
/// Returns a [Result](../../../models/api/result/struct.Result.html)
/// <[Route](../../../models/route/struct.Route.html)>
#[get("/routes/<route_uid>")]
pub fn route_by_id(rh: State<RoutesHandler>, route_uid: String) -> Json<Result<Route>>{
    let query = "SELECT * FROM route WHERE uid = $1 LIMIT 1";
    let conn = rh.pool.clone().get().unwrap();
    let routes = conn.query(
        query,
        &[&route_uid]
    );

    let routes = routes.expect("Query failed");

    if routes.len() != 1 {
        return Json(Result{
            result: Option::None,
            meta: Meta {
                success: false,
                error: Some(Error{
                    code: 4,
                    message: String::from("Unable to find this route")
                })
            }
        })
    }

    let route = parse_route_row(&routes.get(0), &rh);

    Json(Result{
        result: Some(route),
        meta: Meta {
            success: true,
            error: Option::None
        }
    })
}

/// `/routes/by-stop/<stop_uid>`  
/// Gets the [Route](../../../models/route/struct.Route.html)s that serve a particular [Stop](../../../models/route/struct.Stop.html) by its UID,
/// parametrized as `<stop_uid>`.  
/// Returns a [Result](../../../models/api/result/struct.Result.html)
/// <[Route](../../../models/route/struct.Route.html)>
#[get("/routes/by-stop/<stop_uid>")]
pub fn route_by_stop_uid(rh: State<RoutesHandler>, stop_uid: String) -> Json<ResultArray<Route>>{
    let query = "SELECT route.uid, route.id, route.agency, route.short_name, route.long_name, route.description, route.\"type\", route.feed_id FROM route, (SELECT trip.route_id as rid, trip.feed_id as fid
    FROM trip, (SELECT trip_id as tid, feed_id as fid FROM stop_time WHERE stop_time.stop_id = (SELECT stop.id FROM stop WHERE stop.uid = $1 )) as sq1
    WHERE sq1.tid = trip.trip_id 
    AND sq1.fid = trip.feed_id
    GROUP BY (trip.route_id, trip.feed_id)) as sq2 WHERE 
    sq2.rid = route.id AND
    sq2.fid = route.feed_id";

    let conn = rh.pool.clone().get().unwrap();
    let routes = conn.query(
        query,
        &[&stop_uid]
    );

    let routes = routes.expect("Query failed");
    let mut results : Vec<Route> = Vec::new();

    for row in routes.iter() {
        results.push(
            parse_route_row(&row, &rh)
        )
    }

    Json(ResultArray{
        result: Some(results),
        meta: Meta {
            success: true,
            error: Option::None
        }
    })
}

fn parse_route_row(row: &Row, rh: &State<RoutesHandler>) -> Route {
    let feed_id : String = row.get(7);
    Route {
        uid: row.get(0),
        id: row.get(1),
        agency_id: agency::get_agency_id(row.get(2), &feed_id, &rh),
        short_name: row.get(3),
        long_name: row.get(4),
        description: row.get(5),
        route_type: row.get(6),
        feed_id
    }
}