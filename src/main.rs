// https://opentransportdata.swiss/en/dataset/timetable-2018-gtfs/permalink
extern crate postgres;
extern crate csv;
extern crate crypto;
extern crate chrono;
extern crate regex;
extern crate r2d2;
extern crate r2d2_postgres;

#[macro_use]
extern crate serde_derive;

use crypto::digest::Digest;
use crypto::sha2::Sha256;

use std::str::FromStr;

use regex::Regex;

use std::fs::File;
use r2d2_postgres::{TlsMode, PostgresConnectionManager};
use r2d2::Pool;

use chrono::{NaiveDate,NaiveTime};
use std::thread;


fn create_pool() -> Pool<PostgresConnectionManager> {
    let manager = PostgresConnectionManager::new(
        "postgres://postgres:mysecretpassword@172.18.0.2:5432",
        TlsMode::None
    ).unwrap();
    let pool = Pool::new(manager).unwrap();
    pool
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
struct StopTimeCSV {
    trip_id: String,
    arrival_time: String,
    departure_time: String,
    stop_id: String,
    stop_sequence: i32,
    pickup_type: i32,
    drop_off_type: i32
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

fn parse_agency<'a>(feed_id: &str, path: &str, pool: &Pool<PostgresConnectionManager>) {
    let f = File::open(path).expect("File not found");
    let mut rdr = csv::Reader::from_reader(f);

    for result in rdr.deserialize() {
        let conn = pool.clone().get().unwrap();
        let feed_clone : String = String::from(feed_id).to_owned();
        thread::spawn(move || {
            let stmt = conn.prepare("INSERT INTO agency \
                (id, name, url, timezone, lang, phone, feed_id)\
                VALUES($1, $2, $3, $4, $5, $6, $7)"
            ).expect("Unable to create statement");
            let record: AgencyCSV = result.unwrap();
            println!("{}", record.agency_id);
            stmt.execute(&[
                &record.agency_id,
                &record.agency_name,
                &record.agency_url,
                &record.agency_timezone,
                &record.agency_lang,
                &record.agency_phone,
                &feed_clone
            ]).expect("Unable to add agency");
        });
    }
}

fn parse_stops(feed_id: &str, path: &str, pool: &Pool<PostgresConnectionManager>){
    let f = File::open(path).expect("File not found");
    let mut rdr = csv::Reader::from_reader(f);
    for result in rdr.deserialize() {
        let conn = pool.clone().get().unwrap();
        let feed_clone : String = String::from(feed_id).to_owned();

        thread::spawn(move || {
            let record: StopCSV = result.unwrap();
            let stmt = conn.prepare("INSERT INTO stop\
                (id, name, position, type, parent_stop, feed_id)\
                VALUES ($1, $2, ST_GeographyFromText($3), $4, $5, $6)\
                ON CONFLICT DO NOTHING"
            ).expect("Unable to create statement");

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
                &feed_clone
            ]).expect("Cannot add stop");
        });

        //println!("{}", record.stop_name);
    }
}

fn parse_routes(feed_id: &str, path: &str, pool: &Pool<PostgresConnectionManager>){
    let f = File::open(path).expect("File not found");
    let mut rdr = csv::Reader::from_reader(f);
    for result in rdr.deserialize() {
        let conn = pool.clone().get().unwrap();
        let feed_clone : String = String::from(feed_id).to_owned();

        thread::spawn(move || {
            let record: RouteCSV = result.unwrap();
            let stmt = conn.prepare(
                "INSERT INTO route (id, agency, short_name, long_name, description, type, feed_id)\
                VALUES ($1, $2, $3, $4, $5, $6, $7)"
            ).expect("Unable to create statement");
            println!("Route ID {}, {}, {} ({})",
                record.route_id,
                record.route_short_name,
                record.route_long_name,
                record.route_type
            );

            stmt.execute(&[
                &record.route_id,
                &record.agency_id,
                &record.route_short_name,
                &record.route_long_name,
                &record.route_desc,
                &(match record.route_type.parse::<i32>() {
                    Ok(v) => {v},
                    Err(_e) => {0}   
                }),
                &feed_clone
            ]).expect("Unable to insert route");
        });
    }
}

fn parse_trips(feed_id: &str, path: &str, pool: &Pool<PostgresConnectionManager>){
    let f = File::open(path).expect("File not found");
    let mut rdr = csv::Reader::from_reader(f);
    for result in rdr.deserialize() {
        let conn = pool.clone().get().unwrap();
        let feed_clone : String = String::from(feed_id).to_owned();

        thread::spawn(move || {
            let stmt = conn.prepare("INSERT INTO trip (\
                    route_id,
                    service_id,
                    headsign,
                    short_name,
                    direction_id,
                    feed_id
                )\
                VALUES ($1, $2, $3, $4, $5, $6)"
            ).expect("Unable to create statement");
            let record: TripCSV = result.unwrap();
            println!("Trip {}",
                    record.trip_headsign
            );

            stmt.execute(&[
                &record.route_id,
                &record.service_id,
                &record.trip_headsign,
                &record.trip_short_name,
                &record.direction_id,
                &feed_clone
            ]);
        });
    }
}

fn parse_stop_times(feed_id: &str, path: &str, pool: &Pool<PostgresConnectionManager>){
    let f = File::open(path).expect("File not found");
    let mut rdr = csv::Reader::from_reader(f);
    for result in rdr.deserialize() {
        let conn = pool.get().unwrap();
        let feed_clone : String = String::from(feed_id).to_owned();
        
        thread::spawn(move || {
            let record: StopTimeCSV = result.unwrap();
            let stmt = conn.prepare("INSERT INTO stop_time (\
                    trip_id,
                    arrival_time,
                    departure_time,
                    stop_id,
                    stop_sequence,
                    pickup_type,
                    drop_off_type,
                    feed_id
                )\
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
            ).expect("Unable to create statement");
            /*println!("Stop time {}-{}",
                    record.stop_id,
                    record.trip_id
            );*/

            let re = Regex::new(r"^(\d{2}):(\d{2}):(\d{2})$").unwrap();
            if !re.is_match(&record.arrival_time){
                panic!("Invalid arrival time!");
            }

            if !re.is_match(&record.departure_time){
                panic!("Invalid departure time!");
            }      

            let  (mut h_arr, mut m_arr, mut s_arr) = (0,0,0);
            let  (mut h_dep, mut m_dep, mut s_dep) = (0,0,0);  

            for cap in re.captures_iter(&record.arrival_time){
                h_arr = u32::from_str(&cap[1]).unwrap() % 24;
                m_arr = u32::from_str(&cap[2]).unwrap();
                s_arr = u32::from_str(&cap[3]).unwrap();
            }

            for cap in re.captures_iter(&record.departure_time){
                h_dep = u32::from_str(&cap[1]).unwrap() % 24;
                m_dep = u32::from_str(&cap[2]).unwrap();
                s_dep = u32::from_str(&cap[3]).unwrap();
            }


            let arr_time = NaiveTime::from_hms(
                h_arr,
                m_arr,
                s_arr
            );

            let dep_time = NaiveTime::from_hms(
                h_dep,
                m_dep,
                s_dep
            );


            stmt.execute(&[
                &record.trip_id,
                &arr_time,
                &dep_time,
                &record.stop_id,
                &record.stop_sequence,
                &record.pickup_type,
                &record.drop_off_type,
                &feed_clone
            ]).expect("Unable to insert stop time");
        });
    }
}

fn parse_feed(path: &str, pool: &Pool<PostgresConnectionManager>) -> String {
    let f = File::open(path).expect("File not found");
    let mut rdr = csv::Reader::from_reader(f);


    for result in rdr.deserialize() {
        let conn = pool.clone().get().unwrap();
        let stmt = conn.prepare("INSERT INTO feed \
                (id, publisher_name, publisher_url, lang,\
                start_date, end_date, version) \
                VALUES ($1, $2, $3, $4, $5, $6, $7)"
        ).expect("Unable to create statement");
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
        ]).expect("Unable to add feed");

        return feed_id;
    }

    return String::new();
}

fn create_tables(pool : &Pool<PostgresConnectionManager>){

    let conn = pool.clone().get().unwrap();
    conn.execute("CREATE TABLE IF NOT EXISTS feed
    (\
        id VARCHAR(64) PRIMARY KEY NOT NULL,\
        publisher_name VARCHAR(255),\
        publisher_url VARCHAR(255),\
        lang VARCHAR(20),
        start_date DATE NOT NULL,
        end_date DATE NOT NULL,
        version VARCHAR(255)
    )", &[]).expect("Cannot create table \"feed\"");

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
    )", &[]).expect("Cannot create table \"agency\"");


    conn.execute("CREATE TABLE IF NOT EXISTS stop\
    (\
        id VARCHAR(255) NOT NULL,\
        name VARCHAR(255) NOT NULL,\
        position GEOGRAPHY(POINT,4326),
        type INT,
        parent_stop VARCHAR(255),
        feed_id VARCHAR(64) NOT NULL,
        PRIMARY KEY (id, feed_id)
    )", &[]).expect("Cannot create table \"stop\"");

    conn.execute("CREATE TABLE IF NOT EXISTS route\
    (\
        id VARCHAR(255) NOT NULL,\
        agency VARCHAR(255) NOT NULL,\
        short_name VARCHAR(255) NOT NULL,\
        long_name VARCHAR(255) NOT NULL,\
        description VARCHAR(255),\
        type INTEGER,
        feed_id VARCHAR(64) NOT NULL,\
        PRIMARY KEY (id, feed_id)\
    )", &[]).expect("Cannot create table \"route\"");


    conn.execute("CREATE TABLE IF NOT EXISTS trip\
    (\
        route_id VARCHAR(255) NOT NULL,\
        service_id VARCHAR(255) NOT NULL,\
        headsign VARCHAR(255) NOT NULL,\
        short_name VARCHAR(255) NOT NULL,\
        direction_id INTEGER,
        feed_id VARCHAR(64) NOT NULL,\
        PRIMARY KEY (route_id, service_id, feed_id)\
    )", &[]).expect("Cannot create table \"trip\"");

    conn.execute("CREATE TABLE IF NOT EXISTS stop_time\
    (\
        trip_id VARCHAR(255) NOT NULL,\
        arrival_time TIME NOT NULL,\
        departure_time TIME NOT NULL,\
        stop_id VARCHAR(255) NOT NULL,\
        stop_sequence INTEGER,
        pickup_type INTEGER,
        drop_off_type INTEGER,
        feed_id VARCHAR(64) NOT NULL,\
        PRIMARY KEY (trip_id, stop_id, stop_sequence, feed_id)\
    )", &[]).expect("Cannot create table \"trip\"");
}

fn stops_near(pool: &Pool<PostgresConnectionManager>, lat: f32, lng: f32, meters: f64){
    let query = "SELECT 
        id, 
        name, 
        type, 
        parent_stop, 
        feed_id,
        ST_Y(position::geometry) as lat,
        ST_X(position::geometry) as lng FROM stop WHERE \
        ST_Distance(position, \
        ST_GeomFromText($1)) <= $2;";

    //println!(format!("{}", query));
    let conn = pool.clone().get().unwrap();
    let stops = conn.query(
        query,
    &[
            &format!("POINT({:.5} {:.5})", lng, lat),
            &meters
        ]
    );

    for row in stops.expect("Query failed").iter() {
        //let a : String = row.get(2);
        let lat : f64 = row.get(5);
        let lng : f64 = row.get(6);

        let stop = Stop {
            id: row.get(0),
            name: row.get(1),
            lat: lat,
            lng: lng,
            location_type: row.get(2),
            parent_station: row.get(3),
            feed_id: row.get(4)

        };
        println!("{:?}", stop);
    }
}

fn main() {
    let pool = create_pool();
    create_tables(&pool);

    let feed_id = parse_feed("./resources/gtfs/sbb/feed_info.txt", &pool);
    println!("{}", feed_id);
    /*parse_agency(
        &feed_id,
        "./resources/gtfs/sbb/agency.txt",
        &conn
    );*/

    stops_near(&pool, 46.00598, 8.952449, 200.0);
    //parse_stops(&feed_id, "./resources/gtfs/sbb/stops.txt", &conn);
    //parse_routes(&feed_id, "./resources/gtfs/sbb/routes.txt", &conn);
    //parse_trips(&feed_id, "./resources/gtfs/sbb/trips.txt", &conn);
    parse_stop_times(&feed_id, "./resources/gtfs/sbb/stop_times.txt", &pool);
}
