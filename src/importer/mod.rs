extern crate csv;
extern crate crypto;
extern crate chrono;
extern crate regex;
extern crate r2d2;
extern crate r2d2_postgres;


pub mod importer {
    use importer::crypto::digest::Digest;
    use importer::crypto::sha2::Sha256;

    use std::io::BufReader;
    use std::str::FromStr;

    use importer::regex::Regex;

    use std::fs::File;
    use importer::r2d2_postgres::{TlsMode, PostgresConnectionManager};
    use importer::r2d2::Pool;

    use importer::chrono::{NaiveDate,NaiveTime};
    use std::thread;
    
    use models::csv::agency::AgencyCSV;
    use models::csv::feed::FeedCSV;
    use models::csv::route::RouteCSV;
    use models::csv::stop::StopCSV;
    use models::csv::stoptime::StopTimeCSV;
    use models::csv::trip::TripCSV;

    use importer::csv;

    fn parse_agency(feed_id: &str, path: &str, pool: &Pool<PostgresConnectionManager>) {
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
                        Err(_e) => 0
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
                ]).expect("Unable to parse trip");
            });
        }
    }

    fn parse_stop_times(feed_id: &str, path: &str, pool: &Pool<PostgresConnectionManager>){
        let f = File::open(path).expect("File not found");
        let mut reader = BufReader::new(f);
        let mut rdr = csv::Reader::from_reader(reader);
        for result in rdr.byte_records() {
            let record = result.unwrap();
            let conn = pool.get().unwrap();
            let feed_clone : String = String::from(feed_id).to_owned();
            
            thread::spawn(move || {
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
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8) ON CONFLICT DO NOTHING"
                ).expect("Unable to create statement");

                let re = Regex::new(r"^(\d{2}):(\d{2}):(\d{2})$").unwrap();
                let at = &String::from_utf8(record[1].to_vec()).unwrap();
                let dt = &String::from_utf8(record[2].to_vec()).unwrap();

                if !re.is_match(at) {
                    panic!("Invalid arrival time!");
                }

                if !re.is_match(dt) {
                    panic!("Invalid departure time!");
                }      

                let  (mut h_arr, mut m_arr, mut s_arr) = (0,0,0);
                let  (mut h_dep, mut m_dep, mut s_dep) = (0,0,0);  

                for cap in re.captures_iter(at){
                    h_arr = u32::from_str(&cap[1]).unwrap() % 24;
                    m_arr = u32::from_str(&cap[2]).unwrap();
                    s_arr = u32::from_str(&cap[3]).unwrap();
                }

                for cap in re.captures_iter(dt){
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
                    &String::from_utf8(record[0].to_vec()).unwrap(),
                    &arr_time,
                    &dep_time,
                    &String::from_utf8(record[3].to_vec()).unwrap(),
                    &String::from_utf8(record[4].to_vec()).unwrap().parse::<i32>().unwrap(),
                    &String::from_utf8(record[5].to_vec()).unwrap().parse::<i32>().unwrap(),
                    &String::from_utf8(record[6].to_vec()).unwrap().parse::<i32>().unwrap(),
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
                    VALUES ($1, $2, $3, $4, $5, $6, $7) ON CONFLICT DO NOTHING"
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
}