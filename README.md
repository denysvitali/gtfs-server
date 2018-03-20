# GTFS Server
A [General Transit Feed Specification (GTFS)](https://en.wikipedia.org/wiki/General_Transit_Feed_Specification) server to expose
some REST APIs.

## Requirements
- Rust (Nightly, preferably)  
- PostgreSQL with GIS ([PostGIS](http://www.postgis.org/))
- (Docker)

### Data Visualization (not required)
- QGIS

## Instructions

### Download & Extract a feed
```bash
mkdir resources/gtfs/sbb/
wget https://opentransportdata.swiss/en/dataset/timetable-2018-gtfs/permalink -O resources/gtfs/sbb/gtfs.zip
cd resources/gtfs/sbb/
unzip gtfs.zip
```

### Deploy with Docker
#### PostGIS
```
docker network create --subnet=172.18.0.0/16 gtfs-server-net
docker run --name gtfs-server-db --net gtfs-server-net --ip 172.18.0.2 -e POSTGRES_PASSWORD=mysecretpassword -d mdillon/postgis/
```

### Run the server
```
cargo run
```

### Check if the data was imported
http://127.0.0.1:8080/api/stops

### Data import
`feed-id` is your feed unique identifier. It will be used across the DB to generate the stop IDs. 
This will allow us to filter out the feeds that are no longer actives once the DB is populated.
#### Stops
http://127.0.0.1:8080/api/import/stops/feed-id

#### Trips
http://127.0.0.1:8080/api/import/stops/feed-id


## Endpoints & Objects
Check the [Documentation](https://denysvitali.github.io/gtfs-server/) for more info about the endpoints and the objects.

## Screenshots

### /api/trips/`<stop_id>`
![/api/trips/<stop_id>](screenshots/1.png)
### /api/stops/
![/api/stops/](screenshots/2.png)
### /api/stops/near/`<latitude>`/`<longitude>`/`<range>`
![/api/stops/near/<latitude>/<longitude>/<range>](screenshots/3.png)