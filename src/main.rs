// https://opentransportdata.swiss/en/dataset/timetable-2018-gtfs/permalink
extern crate postgres;
extern crate csv;
#[macro_use]
extern crate serde_derive;

use std::fs::File;
use postgres::{Connection, TlsMode};

fn conn(){
    let conn = Connection::connect(
        "postgres://postgres:mysecretpassword@172.18.0.2:5432",
        TlsMode::None
    ).unwrap();

    conn.execute("CREATE TABLE test (id int PRIMARY KEY, name VARCHAR NOT NULL)", &[]).unwrap();
}

#[derive(Debug,Deserialize)]
struct Agency {
    agency_id: String,
    agency_name: String,
    agency_url: String,
    agency_timezone: String,
    agency_lang: String,
    agency_phone: String
}

#[derive(Debug,Deserialize)]
struct Stop {
    stop_id: String,
    stop_name: String,
    stop_lat: f32,
    stop_lon: f32,
    location_type: String,
    parent_station: String
}

#[derive(Debug,Deserialize)]
struct Route {
    route_id: String,
    agency_id: String,
    route_short_name: String,
    route_long_name: String,
    route_desc: String,
    route_type: String
}

#[derive(Debug,Deserialize)]
struct Trip {
    route_id: String,
    service_id: String,
    trip_id: String,
    trip_headsign: String,
    trip_short_name: String,
    direction_id: i32
}

fn parse_agency(path: &str){
    let f = File::open(path).expect("File not found");
    let mut rdr = csv::Reader::from_reader(f);
    for result in rdr.deserialize() {
        let record: Agency = result.unwrap();
        println!("{}", record.agency_id);
    }
}

fn parse_stops(path: &str){
    let f = File::open(path).expect("File not found");
    let mut rdr = csv::Reader::from_reader(f);
    for result in rdr.deserialize() {
        let record: Stop = result.unwrap();
        println!("{}", record.stop_name);
    }
}

fn parse_routes(path: &str){
    let f = File::open(path).expect("File not found");
    let mut rdr = csv::Reader::from_reader(f);
    for result in rdr.deserialize() {
        let record: Route = result.unwrap();
        println!("Route ID {}, {}, {} ({})",
            record.route_id,
            record.route_short_name,
            record.route_long_name,
            record.route_type
        );
    }
}

fn parse_trips(path: &str){
    let f = File::open(path).expect("File not found");
    let mut rdr = csv::Reader::from_reader(f);
    for result in rdr.deserialize() {
        let record: Trip = result.unwrap();
        println!("Trip {}",
                 record.trip_headsign
        );
    }
}

fn main() {
    //conn();
    //parse_agency("./resources/gtfs/sbb/agency.txt");
    //parse_stops("./resources/gtfs/sbb/stops.txt");
    //parse_routes("./resources/gtfs/sbb/routes.txt");
    parse_trips("./resources/gtfs/sbb/trips.txt");
}
