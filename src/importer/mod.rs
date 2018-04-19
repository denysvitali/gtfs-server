extern crate chrono;
extern crate crypto;
extern crate csv;
extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate regex;
extern crate reqwest;
extern crate tempfile;
extern crate zip;

pub extern crate serde;
pub extern crate serde_json;

pub use self::serde::de as serde_de;
pub use self::serde::Deserializer;

use self::crypto::digest::Digest;
use self::crypto::sha2::Sha256;

use std::io::BufReader;
use std::str::FromStr;

use self::regex::Regex;

use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use postgres::rows::{Row, Rows};
use postgres::Connection;
use std::fs::File;
use importer::zip::read::ZipArchive;
use importer::zip::read::ZipFile;

pub use self::chrono::{NaiveDate, NaiveTime};
use std::thread;
use std::process::Command;
use std::env;

use models::csv::agency::AgencyCSV;
use models::csv::calendar::CalendarCSV;
use models::csv::feed::FeedCSV;
use models::csv::route::RouteCSV;
use models::csv::stop::StopCSV;
use models::csv::trip::TripCSV;
use self::csv::Reader;
use std::io::Read;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn download_feed_zip<'a>(feed_url: &str, pool: &Pool<PostgresConnectionManager>) -> Option<ZipArchive<File>> {
    // Download feed from URL
    // Example:  https://opentransportdata.swiss/en/dataset/timetable-2018-gtfs/permalink
    let mut resp = reqwest::get(feed_url).expect("Download failed");
    let mut tmpfile : File = tempfile::tempfile().expect("Unable to get a tmp file");
    //let mut tmpdir = tempfile::tempdir().expect("Unable to create a tmp dir");

    let result = resp.copy_to(&mut tmpfile);
    println!("Result: {:?}", result);
    if result.is_ok() {
        println!("Result is ok");
        // We've got a zip file!
        let mut zar : ZipArchive<File> = ZipArchive::new(tmpfile)
            .expect("This is not a zip file");
        return Some(zar);
    }
    return Option::None;
}

struct MZipFile<'a> {
    pub name : &'a str,
    pub order: i32
}

impl<'a> PartialEq for MZipFile<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.order == other.order
    }
}

impl<'a> Eq for MZipFile<'a> {}

impl<'a> Ord for MZipFile<'a>{
    fn cmp(&self, other: &Self) -> Ordering {
        if self.order > other.order {
            return Ordering::Greater;
        } else if self.order < other.order {
            return Ordering::Less;
        }
        return Ordering::Equal;
    }

    fn max(self, other: Self) -> Self {
        if self.order > other.order {
            return self;
        }
        return other;
    }

    fn min(self, other: Self) -> Self {
        if self.order > other.order {
            return other;
        }
        return self;
    }
}

impl<'a> PartialOrd for MZipFile<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn parse_feed_zip(zar: &mut ZipArchive<File>, pool: &Pool<PostgresConnectionManager>){
    let mut feed_id : String = String::new();
        for i in 0..zar.len() {
            let mut file = zar.by_index(i).expect("Invalid file index");
            println!("File {} ", file.name());
            if file.name() == "feed_info.txt" {
                feed_id = parse_feed(file, pool);
                break;
            }
        }

        if feed_id.len() == 0 {
            println!("feed_info missing!");
            let start = SystemTime::now();
            let uts = start.duration_since(UNIX_EPOCH).unwrap();
            let mut sha = Sha256::new();
            sha.input_str(&uts.as_secs().to_string());
            feed_id = sha.result_str();
            println!("Using fake feed ID {}", feed_id);
            //return;
        }

        let mut bh : BinaryHeap<MZipFile> = BinaryHeap::new();
        for i in 0..zar.len() {
            let file = zar.by_index(i).expect("Invalid file index");
            println!("File {} ", file.name());
            match file.name() {
                "agency.txt" => {
                    bh.push(MZipFile{
                        name: "agency.txt",
                        order: 1
                    })
                },
                "calendar.txt" => {
                    bh.push(MZipFile{
                        name: "calendar.txt",
                        order: 4
                    })
                },
                "routes.txt" => {
                    bh.push(MZipFile{
                        name: "routes.txt",
                        order: 2
                    })
                },
                "stops.txt" => {
                    bh.push(MZipFile{
                        name: "stops.txt",
                        order: 2
                    })
                },
                "stop_times.txt" => {
                    bh.push(MZipFile{
                        name: "stop_times.txt",
                        order: 5
                    })
                },
                "trips.txt" => {
                    bh.push(MZipFile{
                        name: "trips.txt",
                        order: 3
                    })
                },
                _ => {}
            };
        }
        
        println!("Parsing BH");
        for i in bh.into_sorted_vec() {
            println!("{}", i.name);
            match i.name {
                "agency.txt" => {
                    parse_agency(&feed_id, zar.by_name(i.name).unwrap(), &pool)
                },
                "calendar.txt" => {
                    parse_calendar(&feed_id, zar.by_name(i.name).unwrap(), &pool)
                },
                "routes.txt" => {
                    parse_routes(&feed_id, zar.by_name(i.name).unwrap(), &pool)
                },
                "stops.txt" => {
                    parse_stops(&feed_id, zar.by_name(i.name).unwrap(), &pool)
                },
                "stop_times.txt" => {
                    parse_stop_times(&feed_id, zar.by_name(i.name).unwrap(), &pool)
                },
                "trips.txt" => {
                    parse_trips(&feed_id, zar.by_name(i.name).unwrap(), &pool)
                },
                _ => {}
            };
        }

        
}

pub fn parse_agency(feed_id: &str, f: ZipFile, pool: &Pool<PostgresConnectionManager>) {
    println!("Parsing agency for fid {} ...", feed_id);
    let mut rdr = csv::ReaderBuilder::new().from_reader(f);
    for result in rdr.deserialize() {
        let conn = pool.clone().get().unwrap();
        let feed_clone: String = String::from(feed_id).to_owned();
        thread::spawn(move || {
            let stmt = conn.prepare(
                "INSERT INTO agency \
                (
                    uid, 
                    id, 
                    name, 
                    url, 
                    timezone, 
                    lang, 
                    phone, 
                    fare_url,
                    email,
                    feed_id
                ) \
                VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) \
                ON CONFLICT DO NOTHING",
            ).expect("Unable to create statement");

            let record: AgencyCSV = result.unwrap();
            let mut agency_id = String::from("1");

            if record.agency_id.is_some() {
                agency_id = record.agency_id.unwrap();
            }

            let uid = generate_uid(
                "a",
                &format!("{}{}", &agency_id, feed_clone),
                &record.agency_name,
            );

            println!("{}", &agency_id);
            stmt.execute(&[
                &uid,
                &agency_id,
                &record.agency_name,
                &record.agency_url,
                &record.agency_timezone,
                &record.agency_lang,
                &record.agency_phone,
                &record.agency_fare_url,
                &record.agency_email,
                &feed_clone,
            ]).expect("Unable to add agency");
        });
    }
}

fn generate_uid(identifier: &str, fields: &str, name: &str) -> String {
    let mut sha = Sha256::new();
    sha.input_str(fields);
    let mut stopsha: String = sha.result_str();
    &stopsha.truncate(6);

    let re = Regex::new(r"[^A-Za-z0-9]").unwrap();
    let name_stripped = Regex::new(r"\s").unwrap().replace_all(name, "");
    let name_stripped = Regex::new(r"[(),.-]")
        .unwrap()
        .replace_all(&name_stripped, "");
    let uid = re.replace_all(&name_stripped, "-")
        .to_owned()
        .to_lowercase();
    let uid = format!("{}-{}-{}", identifier, &stopsha, uid);
    uid
}

pub fn parse_stops(feed_id: &str, f: ZipFile, pool: &Pool<PostgresConnectionManager>) {
    println!("Parsing stops for fid {} ...", feed_id);
    let mut rdr = csv::ReaderBuilder::new().from_reader(f);
    for result in rdr.deserialize() {
        let conn = pool.clone().get().unwrap();
        let feed_clone: String = String::from(feed_id).to_owned();

        thread::spawn(move || {
            let record: StopCSV = result.unwrap();
            let stmt = conn.prepare(
                "INSERT INTO stop\
                 (
                     uid, 
                     id, 
                     code, 
                     name, 
                     description, 
                     position,
                     zone_id, 
                     url,  
                     type, 
                     parent_stop, 
                     timezone, 
                     wheelchair_boarding,  
                     feed_id
                )\
                VALUES (
                    $1, $2, $3, $4, $5, ST_GeographyFromText($6), $7,
                    $8, $9, $10, $11, $12, $13
                ) \
                ON CONFLICT DO NOTHING",
            ).expect("Unable to create statement");

            let uid = generate_uid(
                "s",
                &format!("{}{}{}", feed_clone, &record.stop_name, &record.stop_id),
                &record.stop_name,
            );

            stmt.execute(&[
                &uid,
                &record.stop_id,
                &record.stop_code,
                &record.stop_name,
                &record.stop_desc,
                &format!("SRID=4326;POINT({} {})", record.stop_lon, record.stop_lat),
                &record.zone_id,
                &record.stop_url,
                &record.location_type,
                &record.parent_station,
                &record.stop_timezone,
                &record.wheelchair_boarding,
                &feed_clone,
            ]).expect("Cannot add stop");
        });
    }
}

pub fn parse_routes(feed_id: &str, f: ZipFile, pool: &Pool<PostgresConnectionManager>) {
    println!("Parsing routes for fid {} ...", feed_id);
    let mut rdr = csv::ReaderBuilder::new().from_reader(f);
    for result in rdr.deserialize() {
        let conn = pool.clone().get().unwrap();
        let feed_clone: String = String::from(feed_id).to_owned();

        thread::spawn(move || {
            let record: RouteCSV = result.unwrap();
            let stmt = conn.prepare(
                "INSERT INTO route (
                    uid, 
                    id, 
                    agency, 
                    short_name, 
                    long_name, 
                    description, 
                    type, 
                    url,
                    color,
                    text_color,
                    sort_order,
                    feed_id
                ) \
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12) \
                ON CONFLICT DO NOTHING"
            ).expect("Unable to create statement");
            println!(
                "Route ID {}, {}, {} ({})",
                record.route_id, record.route_short_name, record.route_long_name, record.route_type
            );

            let uid = generate_uid(
                "r",
                &format!("{}{}", record.route_id, feed_clone),
                &record.route_short_name,
            );

            stmt.execute(&[
                &uid,
                &record.route_id,
                &record.agency_id,
                &record.route_short_name,
                &record.route_long_name,
                &record.route_desc,
                &record.route_type,
                &record.route_url,
                &record.route_color,
                &record.route_text_color,
                &record.route_sort_order,
                &feed_clone,
            ]).expect("Unable to insert route");
        });
    }
}

pub fn parse_trips(feed_id: &str, f: ZipFile, pool: &Pool<PostgresConnectionManager>) {
    println!("Parsing trips for fid {} ...", feed_id);
    let mut rdr = csv::ReaderBuilder::new().from_reader(f);
    let mut fields : Vec<String> = Vec::new();
    {
        for i in rdr.byte_headers().unwrap().iter() {
            fields.push(String::from_utf8(i.to_vec()).unwrap());
        }
    }
    for result in rdr.deserialize() {
        let conn = pool.clone().get().unwrap();
        let feed_clone: String = String::from(feed_id).to_owned();
        let fc = fields.clone();
        thread::spawn(move || {
            let stmt = conn.prepare(
                "INSERT INTO trip (\
                    uid,
                    route_id,
                    service_id,
                    trip_id,
                    headsign,
                    short_name,
                    direction_id,
                    block_id,
                    shape_id,
                    wheelchair_accessible,
                    bikes_allowed,
                    feed_id
                )\
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9,
                $10, $11, $12) \
                ON CONFLICT DO NOTHING",
            ).expect("Unable to create statement");
            let record: TripCSV = result.unwrap();

            let uid = generate_uid(
                "t",
                &format!(
                    "{}{}",
                    feed_clone, &record.trip_id
                ),
                &record.trip_id,
            );

            stmt.execute(&[
                &uid,
                &record.route_id,
                &record.service_id,
                &record.trip_id,
                &record.trip_headsign,
                &record.trip_short_name,
                &record.direction_id,
                &record.block_id,
                &record.shape_id,
                &record.wheelchair_accessible,
                &record.bikes_allowed,
                &feed_clone,
            ]).expect("Unable to parse trip");
        });
    }
}

fn field_index_by_name(field_name: &str, fields: &Vec<String>) -> Option<usize> {
    let mut i : usize = 0;
    for f in fields {
        if field_name == f {
            return Some(i);
        }
        i+= 1;
    }
    Option::None
}

pub fn parse_stop_times(feed_id: &str, f: ZipFile, pool: &Pool<PostgresConnectionManager>) {
    println!("Parsing stop times for fid {} ...", feed_id);
    let reader = BufReader::new(f);
    let mut rdr = csv::Reader::from_reader(reader);
    let mut fields : Vec<String> = Vec::new();
    {
        for i in rdr.byte_headers().unwrap().iter() {
            fields.push(String::from_utf8(i.to_vec()).unwrap());
        }
    }
    println!("{:?}", fields);
    for result in rdr.byte_records() {
        let record = result.unwrap();
        let conn = pool.get().unwrap();
        let feed_clone: String = String::from(feed_id).to_owned();
        let fc = fields.clone();

        thread::spawn(move || {
            let stmt = conn.prepare(
                "INSERT INTO stop_time (\
                    trip_id,
                    arrival_time,
                    departure_time,
                    stop_id,
                    stop_sequence,
                    stop_headsign,
                    pickup_type,
                    drop_off_type,
                    shape_dist_traveled,
                    timepoint,
                    feed_id
                )\
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11) ON CONFLICT DO NOTHING",
            ).expect("Unable to create statement");

            let re = Regex::new(r"^(\d{2}):(\d{2}):(\d{2})$").unwrap();
            let at = &String::from_utf8(record[field_index_by_name("arrival_time", &fc).unwrap()].to_vec()).unwrap();
            let dt = &String::from_utf8(record[field_index_by_name("departure_time", &fc).unwrap()].to_vec()).unwrap();

            if !re.is_match(at) {
                panic!("Invalid arrival time!");
            }

            if !re.is_match(dt) {
                panic!("Invalid departure time!");
            }

            let (mut h_arr, mut m_arr, mut s_arr) = (0, 0, 0);
            let (mut h_dep, mut m_dep, mut s_dep) = (0, 0, 0);

            for cap in re.captures_iter(at) {
                h_arr = u32::from_str(&cap[1]).unwrap() % 24;
                m_arr = u32::from_str(&cap[2]).unwrap();
                s_arr = u32::from_str(&cap[3]).unwrap();
            }

            for cap in re.captures_iter(dt) {
                h_dep = u32::from_str(&cap[1]).unwrap() % 24;
                m_dep = u32::from_str(&cap[2]).unwrap();
                s_dep = u32::from_str(&cap[3]).unwrap();
            }

            let arr_time = NaiveTime::from_hms(h_arr, m_arr, s_arr);
            let dep_time = NaiveTime::from_hms(h_dep, m_dep, s_dep);

            let mut stop_headsign : Option<String> = Option::None;
            let mut pickup_type : Option<i32> = Option::None;
            let mut drop_off_type : Option<i32> = Option::None;
            let mut shape_dist_traveled : Option<f32> = Option::None;
            let mut timepoint : Option<i32> = Option::None;

            // Optional fields
            if field_index_by_name("stop_headsign", &fc).is_some() {
                stop_headsign = Some(
                    String::from_utf8(
                        record[field_index_by_name("stop_headsign", &fc).unwrap()].to_vec()
                    ).unwrap()
                );
            }

            if field_index_by_name("pickup_type", &fc).is_some() {
                pickup_type = Some(String::from_utf8(
                        record[field_index_by_name("pickup_type", &fc).unwrap()].to_vec()).unwrap()
                        .parse::<i32>().unwrap());
            }

            if field_index_by_name("drop_off_type", &fc).is_some() {
                drop_off_type = Some(String::from_utf8(
                        record[field_index_by_name("drop_off_type", &fc).unwrap()].to_vec()).unwrap()
                        .parse::<i32>().unwrap());
            }

            if field_index_by_name("shape_dist_traveled", &fc).is_some() {
                shape_dist_traveled = Some(String::from_utf8(
                        record[field_index_by_name("shape_dist_traveled", &fc).unwrap()].to_vec()).unwrap()
                        .parse::<f32>().unwrap());
            }

            if field_index_by_name("timepoint", &fc).is_some() {
                timepoint = Some(String::from_utf8(
                        record[field_index_by_name("timepoint", &fc).unwrap()].to_vec()).unwrap()
                        .parse::<i32>().unwrap());
            }

            stmt.execute(&[
                &String::from_utf8(record[field_index_by_name("trip_id", &fc).unwrap()].to_vec()).unwrap(),
                &arr_time,
                &dep_time,
                &String::from_utf8(record[field_index_by_name("stop_id", &fc).unwrap()].to_vec()).unwrap(),
                &String::from_utf8(record[field_index_by_name("stop_sequence", &fc).unwrap()].to_vec())
                    .unwrap()
                    .parse::<i32>()
                    .unwrap(),
                &stop_headsign,
                &pickup_type,
                &drop_off_type,
                &shape_dist_traveled,
                &timepoint,
                &feed_clone,
            ]).expect("Unable to insert stop time");
        });
    }
}

fn parse_feed(f: ZipFile, pool: &Pool<PostgresConnectionManager>) -> String {
    println!("Parsing feed...");
    let mut rdr = csv::ReaderBuilder::new().from_reader(f);

    for result in rdr.deserialize() {
        let conn = pool.clone().get().unwrap();
        let stmt = conn.prepare(
            "INSERT INTO feed \
             (id, publisher_name, publisher_url, lang,\
             start_date, end_date, version) \
             VALUES ($1, $2, $3, $4, $5, $6, $7) ON CONFLICT DO NOTHING",
        ).expect("Unable to create statement");
        let record: FeedCSV = result.unwrap();
        println!("Parsing Feed from {}", record.feed_publisher_name);
        let input = format!(
            "{}{}{}{}{}{}",
            record.feed_publisher_name,
            record.feed_publisher_url,
            record.feed_start_date,
            record.feed_end_date,
            record.feed_lang,
            record.feed_version
        );

        let mut sha = Sha256::new();
        sha.input_str(&input);
        let feed_id: String = sha.result_str();

        let start_date = NaiveDate::parse_from_str(&record.feed_start_date, "%Y%m%d").unwrap();

        let end_date = NaiveDate::parse_from_str(&record.feed_end_date, "%Y%m%d").unwrap();

        stmt.execute(&[
            &feed_id,
            &record.feed_publisher_name,
            &record.feed_publisher_url,
            &record.feed_lang,
            &start_date,
            &end_date,
            &record.feed_version,
        ]).expect("Unable to add feed");

        return feed_id;
    }

    return String::new();
}

pub fn parse_calendar(feed_id: &str, f: ZipFile, pool: &Pool<PostgresConnectionManager>) {
    println!("Parsing calendar for fid {} ...", feed_id);
    let mut rdr = csv::Reader::from_reader(f);

    for result in rdr.deserialize() {
        let conn = pool.clone().get().unwrap();
        let stmt = conn.prepare(
            "INSERT INTO calendar \
             (uid, service_id, monday, tuesday, wednesday, thursday,\
             friday, saturday, sunday, start_date, end_date, feed_id) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12) ON CONFLICT DO NOTHING",
        ).expect("Unable to create statement");
        let record: CalendarCSV = result.unwrap();

        let uid = generate_uid(
            "se",
            &format!("{}{}", &record.service_id, &feed_id),
            &record.service_id,
        );

       /* println!(
            "{} - M:{} T:{} W:{} T:{} F:{} S:{} S:{} SD:{} ED:{}",
            record.service_id,
            record.monday,
            record.tuesday,
            record.wednesday,
            record.thursday,
            record.friday,
            record.saturday,
            record.sunday,
            record.start_date,
            record.end_date
        );*/

        let start_date = NaiveDate::parse_from_str(&record.start_date, "%Y%m%d").unwrap();

        let end_date = NaiveDate::parse_from_str(&record.end_date, "%Y%m%d").unwrap();

        stmt.execute(&[
            &uid,
            &record.service_id,
            &record.monday,
            &record.tuesday,
            &record.wednesday,
            &record.thursday,
            &record.friday,
            &record.saturday,
            &record.sunday,
            &start_date,
            &end_date,
            &feed_id,
        ]).expect("Unable to insert calendar entry");
    }
}

pub fn create_tables(pool: &Pool<PostgresConnectionManager>) {
    let conn = pool.clone().get().unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS feed
    (\
        id VARCHAR(64) PRIMARY KEY NOT NULL,\
        publisher_name VARCHAR(255),\
        publisher_url VARCHAR(255),\
        lang VARCHAR(20),
        start_date DATE NOT NULL,
        end_date DATE NOT NULL,
        version VARCHAR(255)
    )",
        &[],
    ).expect("Cannot create table \"feed\"");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS agency\
    (\
        uid VARCHAR(255) NOT NULL,\
        id VARCHAR(255) NOT NULL,\
        name VARCHAR(255) NOT NULL,\
        url VARCHAR(512),\
        timezone VARCHAR(255),\
        lang VARCHAR(20),\
        phone VARCHAR(255),\
        feed_id VARCHAR(64) NOT NULL,
        PRIMARY KEY (id, feed_id)
    )",
        &[],
    ).expect("Cannot create table \"agency\"");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS stop\
    (\
        uid VARCHAR(255) NOT NULL, \
        id VARCHAR(255) NOT NULL,\
        name VARCHAR(255) NOT NULL,\
        position GEOGRAPHY(POINT,4326),
        type INT,
        parent_stop VARCHAR(255),
        feed_id VARCHAR(64) NOT NULL,
        PRIMARY KEY (uid),
        UNIQUE(id, feed_id)
    )",
        &[],
    ).expect("Cannot create table \"stop\"");

    /*
        ALTER TABLE public.stop_time
        ADD CONSTRAINT stop_time_stop_fk
        FOREIGN KEY (stop_id,feed_id)
        REFERENCES public.stop(id,feed_id);
    */

    conn.execute(
        "CREATE TABLE IF NOT EXISTS route\
    (\
        uid VARCHAR(255) NOT NULL,
        id VARCHAR(255) NOT NULL,\
        agency VARCHAR(255),\
        short_name VARCHAR(255) NOT NULL,\
        long_name VARCHAR(255) NOT NULL,\
        description VARCHAR(255),\
        type INTEGER,
        feed_id VARCHAR(64) NOT NULL,\
        PRIMARY KEY (id, feed_id)\
    )",
        &[],
    ).expect("Cannot create table \"route\"");

    /*
        ALTER TABLE public.route
        ADD CONSTRAINT route_agency_fk
        FOREIGN KEY (agency,feed_id)
        REFERENCES public.agency(id,feed_id);
     */

    conn.execute(
        "CREATE TABLE IF NOT EXISTS trip\
    (\
        uid VARCHAR(255) NOT NULL,
        route_id VARCHAR(255) NOT NULL,\
        service_id VARCHAR(255) NOT NULL,\
        trip_id VARCHAR(255) NOT NULL,
        headsign VARCHAR(255) NOT NULL,\
        short_name VARCHAR(255) NOT NULL,\
        direction_id INTEGER,
        block_id INTEGER,
        shape_id INTEGER,
        wheelchair_accessible BOOLEAN,
        bikes_allowed BOOLEAN,
        feed_id VARCHAR(64) NOT NULL,\
        PRIMARY KEY (uid)\
    )",
        &[],
    ).expect("Cannot create table \"trip\"");

    // ALTER TABLE public.trip ADD CONSTRAINT trip_calendar_fk FOREIGN KEY (service_id,feed_id) REFERENCES public.calendar(service_id,feed_id) ;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS stop_time\
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
    )",
        &[],
    ).expect("Cannot create table \"stop_time\"");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS transfer\
    (\
        from_sid VARCHAR(255) NOT NULL,\
        to_sid VARCHAR(255) NOT NULL,\
        transfer_type INTEGER,\
        min_transfer_time INTEGER,
        feed_id VARCHAR(64) NOT NULL,\
        PRIMARY KEY (from_sid, to_sid, feed_id)\
    )",
        &[],
    ).expect("Cannot create table \"transfers\"");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS calendar\
    (\
        uid VARCHAR(255) NOT NULL,
        service_id VARCHAR(255) NOT NULL,\
        monday BOOLEAN,\
        tuesday BOOLEAN,\
        wednesday BOOLEAN,\
        thursday BOOLEAN,\
        friday BOOLEAN,\
        saturday BOOLEAN,\
        sunday BOOLEAN,\
        start_date DATE,\
        end_date DATE,\
        feed_id VARCHAR(64) NOT NULL,
        PRIMARY KEY (uid),
        UNIQUE (service_id, feed_id)\
    )",
        &[],
    ).expect("Cannot create table \"calendar\"");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS settings\
    (\
        key VARCHAR(255),
        value VARCHAR(255),
        PRIMARY KEY (key)\
    )",
        &[],
    ).expect("Cannot create table \"settings\"");

    // Create FKs
    conn.execute(
        "ALTER TABLE public.stop_time
        ADD CONSTRAINT stop_time_stop_fk
        FOREIGN KEY (stop_id,feed_id)
        REFERENCES public.stop(id,feed_id) ON CONFLICT DO NOTHING;",
        &[],
    ).expect("Add stop_time constraints");

    conn.execute(
        "ALTER TABLE public.route
        ADD CONSTRAINT route_agency_fk
        FOREIGN KEY (agency,feed_id)
        REFERENCES public.agency(id,feed_id);",
        &[],
    ).expect("Add stop_time constraints");

    conn.execute(
        "ALTER TABLE public.trip \
         ADD CONSTRAINT trip_calendar_fk \
         FOREIGN KEY (service_id,feed_id) \
         REFERENCES public.calendar(service_id,feed_id) ON CONFLICT DO NOTHING;",
        &[],
    ).expect("Add stop_time constraints");

    // Create Indexes
    conn.execute(
        "CREATE INDEX IF NOT EXISTS stop_time_stop_id_idx ON public.stop_time (stop_id);",
        &[],
    ).expect("Add stop_time index");
    conn.execute(
        "CREATE UNIQUE INDEX IF NOT EXISTS  stop_id_idx ON public.stop (id,feed_id);",
        &[],
    ).expect("Add stop index");
    conn.execute(
        "CREATE UNIQUE INDEX IF NOT EXISTS  route_id_idx ON public.route (id,feed_id);",
        &[],
    ).expect("Add route index");
    conn.execute(
        "CREATE UNIQUE INDEX IF NOT EXISTS  calendar_service_id_idx ON public.calendar (service_id,feed_id);",
        &[],
    ).expect("Add calendar index");
    conn.execute(
        "CREATE INDEX IF NOT EXISTS  trip_feed_id_idx ON public.trip (feed_id,trip_id);",
        &[],
    ).expect("Add trip index");
    conn.execute(
        "CREATE INDEX IF NOT EXISTS stop_time_trip_id_idx ON public.stop_time (trip_id,feed_id);",
        &[],
    ).expect("Add stop_time index 2");
    conn.execute(
        "CREATE INDEX IF NOT EXISTS  stop_time_trip_id_feed_idx ON public.stop_time (trip_id,stop_id,feed_id);",
        &[],
    ).expect("Add stop_time index 3");
}

pub fn update_db(pool: &Pool<PostgresConnectionManager>){
    let conn = pool.clone().get().unwrap();
    let db_vers : postgres::Result<Rows> = conn.query("SELECT value FROM settings WHERE key='db_version';", &[]);
    if db_vers.as_ref().is_err() || db_vers.as_ref().is_ok() && db_vers.as_ref().unwrap().len() == 0 {
        println!("Cannot get db version");
        let db_vers : postgres::Result<u64> = conn.execute("SELECT * FROM settings", &[]);
        if db_vers.is_err() {
            // The settings table doesn't exists
            create_tables(&pool);
        }
        conn.execute("INSERT INTO settings (key, value) VALUES('db_version', $1)", &[&"1"])
            .expect("Unable to add db_version to settings");

        update_db_from(1, &pool);
    } else {
        let version : String = db_vers.unwrap().get(0).get(0);
        update_db_from(version.parse::<u32>().unwrap(), &pool);
    }
}

pub fn update_db_ver(ver: u32, conn: &Connection) {
    conn.execute("UPDATE settings SET value=$1 WHERE key = 'db_version'", &[&ver.to_string()]).expect("Version Update Failed");
}

pub fn update_db_from(ver : u32, pool: &Pool<PostgresConnectionManager>){
    let conn = pool.clone().get().unwrap();
    if ver < 2 {
        conn.execute("ALTER TABLE trip \
        ADD COLUMN block_id INTEGER, \
        ADD COLUMN shape_id INTEGER, \
        ADD COLUMN wheelchair_accessible INTEGER, \
        ADD COLUMN bikes_allowed INTEGER", &[]).expect("Unable to alter table trip");
        update_db_ver(2, &conn);
    }

    if ver < 3 {
        conn.execute("ALTER TABLE stop_time \
        ADD COLUMN stop_headsign VARCHAR(255), \
        ADD COLUMN shape_dist_traveled REAL, \
        ADD COLUMN timepoint INTEGER
        ", &[]).expect("Unable to alter table stop_time");
        update_db_ver(3, &conn);
    }

    if ver < 4 {
        conn.execute("ALTER TABLE route \
        ADD COLUMN url VARCHAR(255), \
        ADD COLUMN color CHAR(6), \
        ADD COLUMN text_color CHAR(6), \
        ADD COLUMN sort_order INTEGER", &[])
        .expect("Unable to ALTER TABLE route");
        update_db_ver(4, &conn);
    }

    if ver < 5 {
        conn.execute("ALTER TABLE stop \
        ADD COLUMN description VARCHAR(512), \
        ADD COLUMN code VARCHAR(255), \
        ADD COLUMN zone_id INTEGER, \
        ADD COLUMN url VARCHAR(512), \
        ADD COLUMN timezone VARCHAR(255), \
        ADD COLUMN wheelchair_boarding INTEGER", &[])
        .expect("Unable to ALTER TABLE stop");
        update_db_ver(5, &conn);
    }

    if ver < 6 {
        conn.execute("ALTER TABLE agency \
        ADD COLUMN fare_url VARCHAR(512),
        ADD COLUMN email VARCHAR(255)", &[])
        .expect("Unable to ALTER TABLE agency");
        update_db_ver(6, &conn);
    }

    if ver < 7 {
        conn.execute("ALTER TABLE trip \
        ALTER COLUMN short_name DROP NOT NULL", &[])
        .expect("Unable to ALTER TABLE trip");
        update_db_ver(7, &conn);
    }
}

pub fn get_db_version(pool: &Pool<PostgresConnectionManager>) -> i32 {
    let conn = pool.clone().get().unwrap();
    let result = conn.query("SELECT value FROM settings WHERE key='db_version'", &[]);
    if result.as_ref().is_err() || result.as_ref().unwrap().len() == 0 {
        return -1;
    }

    let db_vers_string : String = result.unwrap().get(0).get(0);
    return db_vers_string.parse::<i32>().unwrap();
}