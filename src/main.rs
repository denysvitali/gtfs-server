// https://opentransportdata.swiss/en/dataset/timetable-2018-gtfs/permalink
extern crate postgres;
extern crate csv;
extern crate crypto;
extern crate chrono;

#[macro_use]
extern crate serde_derive;

use crypto::digest::Digest;
use crypto::sha2::Sha256;

use std::fs::File;
use postgres::{Connection, TlsMode};
use chrono::NaiveDate;


fn create_conn() -> Connection {
    let conn = Connection::connect(
        "postgres://postgres:mysecretpassword@172.18.0.2:5432",
        TlsMode::None
    ).unwrap();
    conn
}

#[derive(Debug,Deserialize)]
struct AgencyCSV {
    agency_id: String,
    agency_name: String,
    agency_url: String,
    agency_timezone: String,
    agency_lang: String,
    agency_phone: String
}

#[derive(Debug,Deserialize)]
struct StopCSV {
    stop_id: String,
    stop_name: String,
    stop_lat: f32,
    stop_lon: f32,
    location_type: String,
    parent_station: String
}

#[derive(Debug,Deserialize)]
struct RouteCSV {
    route_id: String,
    agency_id: String,
    route_short_name: String,
    route_long_name: String,
    route_desc: String,
    route_type: String
}

#[derive(Debug,Deserialize)]
struct TripCSV {
    route_id: String,
    service_id: String,
    trip_id: String,
    trip_headsign: String,
    trip_short_name: String,
    direction_id: i32
}

#[derive(Debug,Deserialize)]
struct FeedCSV {
    feed_publisher_name: String,
    feed_publisher_url: String,
    feed_lang: String,
    feed_start_date: String,
    feed_end_date: String,
    feed_version: String
}

#[derive(Debug,Deserialize)]
struct Stop {
    id: String,
    name: String,
    lat: f64,
    lng: f64,
    location_type: i32,
    parent_station: String,
    feed_id: String
}

fn parse_agency(feed_id: &str, path: &str, conn: &Connection) {
    let f = File::open(path).expect("File not found");
    let mut rdr = csv::Reader::from_reader(f);
    let stmt = conn.prepare("INSERT INTO agency \
            (id, name, url, timezone, lang, phone, feed_id)\
            VALUES($1, $2, $3, $4, $5, $6, $7)"
    ).expect("Unable to create statement");

    for result in rdr.deserialize() {
        let record: AgencyCSV = result.unwrap();
        println!("{}", record.agency_id);
        stmt.execute(&[
            &record.agency_id,
            &record.agency_name,
            &record.agency_url,
            &record.agency_timezone,
            &record.agency_lang,
            &record.agency_phone,
            &feed_id
        ]);
    }
}

fn parse_stops(feed_id: &str, path: &str, conn: &Connection){
    let stmt = conn.prepare("INSERT INTO stop\
        (id, name, position, type, parent_stop, feed_id)\
        VALUES ($1, $2, ST_GeographyFromText($3), $4, $5, $6)\
        ON CONFLICT DO NOTHING"
    ).expect("Unable to create statement");
    let f = File::open(path).expect("File not found");
    let mut rdr = csv::Reader::from_reader(f);
    for result in rdr.deserialize() {
        let record: StopCSV = result.unwrap();
        /*println!("SRID=4326;POINT({} {})",
                         record.stop_lon, record.stop_lat);*/
        stmt.execute(&[
            &record.stop_id,
            &record.stop_name,
            &format!("SRID=4326;POINT({} {})",
                record.stop_lon, record.stop_lat),
            &(match record.location_type.parse::<i32>() {
                Ok(val) => val,
                Err(E) => 0
            }),
            &record.parent_station,
            &feed_id
        ]).expect("Cannot add stop");
        //println!("{}", record.stop_name);
    }
}

fn parse_routes(path: &str){
    let f = File::open(path).expect("File not found");
    let mut rdr = csv::Reader::from_reader(f);
    for result in rdr.deserialize() {
        let record: RouteCSV = result.unwrap();
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
        let record: TripCSV = result.unwrap();
        println!("Trip {}",
                 record.trip_headsign
        );
    }
}

fn parse_feed(path: &str, conn: &Connection) -> String {
    let f = File::open(path).expect("File not found");
    let mut rdr = csv::Reader::from_reader(f);
    let stmt = conn.prepare("INSERT INTO feed \
            (id, publisher_name, publisher_url, lang,\
            start_date, end_date, version) \
            VALUES ($1, $2, $3, $4, $5, $6, $7)"
    ).expect("Unable to create statement");


    for result in rdr.deserialize() {
        let record: FeedCSV = result.unwrap();
        println!("Parsing Feed from {}",
                 record.feed_publisher_name
        );

        let input = format!("{}{}{}{}{}{}",
            record.feed_publisher_name,
            record.feed_publisher_url,
            record.feed_start_date,
            record.feed_end_date,
            record.feed_lang,
            record.feed_version
        );


        let mut sha = Sha256::new();
        sha.input_str(&input);
        let feed_id : String = sha.result_str();

        let start_date =
            NaiveDate::parse_from_str(
                &record.feed_start_date,
                "%Y%m%d"
            ).unwrap();

        let end_date =
            NaiveDate::parse_from_str(
                &record.feed_end_date,
                "%Y%m%d"
            ).unwrap();

        stmt.execute(&[
            &feed_id,
            &record.feed_publisher_name,
            &record.feed_publisher_url,
            &record.feed_lang,
            &start_date,
            &end_date,
            &record.feed_version
        ]);

        return feed_id;
    }

    return String::new();
}

fn create_tables(conn : &Connection){

    conn.execute("CREATE TABLE IF NOT EXISTS feed\
    (\
        id VARCHAR(64) PRIMARY KEY NOT NULL,\
        publisher_name VARCHAR(255),\
        publisher_url VARCHAR(255),\
        lang VARCHAR(20),
        start_date DATE NOT NULL,
        end_date DATE NOT NULL,
        version VARCHAR(255)
    )", &[]);

    conn.execute("CREATE TABLE IF NOT EXISTS agency\
    (\
        id VARCHAR(255) NOT NULL,\
        name VARCHAR(255) NOT NULL,\
        url VARCHAR(512),\
        timezone VARCHAR(255),\
        lang VARCHAR(20),\
        phone VARCHAR(255),\
        feed_id VARCHAR(64) NOT NULL,
        PRIMARY KEY (id, feed_id)
    )", &[]);


    conn.execute("CREATE TABLE IF NOT EXISTS stop\
    (\
        id VARCHAR(255) NOT NULL,\
        name VARCHAR(255) NOT NULL,\
        position GEOGRAPHY(POINT,4326),
        type INT,
        parent_stop VARCHAR(255),
        feed_id VARCHAR(64) NOT NULL,
        PRIMARY KEY (id, feed_id)
    )", &[]).expect("WTF");
}

fn stops_near(conn: &Connection, lat: f32, lng: f32, meters: f64){
    let query = "SELECT
        id,
        name,\
        ST_AsGeoJSON(position) as geojson,\
        type,\
        parent_stop,\
        feed_id\
        FROM stop WHERE \
        ST_Distance(position, \
        ST_GeomFromText($1)) <= $2;";

    println!(format!("{}", query));

    let stops = conn.query(
        query,
    &[
            &format!("POINT({:.5} {:.5})", lng, lat),
            &meters
        ]
    );

    for row in stops.expect("Query failed").iter() {
        let a : String = row.get(2);
        let lat= 0.0;
        let lng = 0.0;

        let stop = Stop {
            id: row.get(0),
            name: row.get(1),
            lat: lat,
            lng: lng,
            location_type: row.get(3),
            parent_station: row.get(4),
            feed_id: row.get(5)

        };
        println!("{:?}", stop);
    }
}

fn main() {
    let conn = create_conn();
    create_tables(&conn);

    let feed_id = parse_feed("./resources/gtfs/sbb/feed_info.txt", &conn);
    println!("{}", feed_id);
    /*parse_agency(
        &feed_id,
        "./resources/gtfs/sbb/agency.txt",
        &conn
    );*/

    stops_near(&conn, 46.00598, 8.952449, 200.0);
    //parse_stops(&feed_id, "./resources/gtfs/sbb/stops.txt", &conn);
    //parse_routes("./resources/gtfs/sbb/routes.txt");
    //parse_trips("./resources/gtfs/sbb/trips.txt");
}
