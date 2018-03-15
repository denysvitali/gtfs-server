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



## Objects

### Stop (s)
A `Stop` represents a physical public transportation stop. 
It may be a Bus Stop, a Train Station, ...

#### Fields

| Field Name | Description |
| ---------- | ----------- |
| uid        | Represents the unique identifier for this stop (`s-[a-f0-9]-[a-z0-9]`),   for example `s-c27ebe-mannolamonda` |
| name       | The name of this stop |
| lat        | Latitude |
| lng        | Longitude |

### Trip (t)

| Field Name | Description |
| ---------- | ----------- |
| uid        | Represents the unique identifier for this stop (`t-[a-f0-9]-[a-z0-9]`),   for example `t-8033c6-bioggiomolinazzostazione` |
| service_id | The Service ID |
| headsign   | This is the heading for the trip, as it would appear on an LED panel |
| short_name | A short name for the trip. For example `713` (aka Bus Number / Line Number) |
| direction_id | TODO: Describe this |

### Meta
This object represents a wrapper for a result.  
In future versions this object will include pagination and offsets, as well as a count of items returned and the limit of results per page.

| Field Name | Description |
| ---------- | ----------- |
| success    | `Boolean`, the request was successful |
| error      | (Optional, only available when success is `true`) An [Error](#error) object. |

### Error

| Field Name | Description |
| ---------- | ----------- |
| code       | An error code - generally this should be unique (one error code per error type). |
| message    | The error message associated - localized in English |



### StopDistance  
A [Stop](#stop-s) with a distance.  

| Field Name | Description |
| ---------- | ----------- |
| stop       | Element of type [Stop](#stop-s) |
| distance   | Float64 representing the distance between this [Stop](#stop-s) and a GPS coordinate specified in a previous request |


## Results
### StopResult

| Field Name | Description |
| ---------- | ----------- |
| result     | Array of [Stop](#stop-s)s |
| meta       | [Meta](#meta) |

### StopDistanceResult

| Field Name | Description |
| ---------- | ----------- |
| result     | Array of [StopDistance](#stop-distance)s |
| meta       | [Meta](#meta) |

### TripResult

| Field Name | Description |
| ---------- | ----------- |
| result     | Array of [Trip](#trip-t)s |
| meta       | [Meta](#meta) |

### SuccessResult

| Field Name | Description |
| ---------- | ----------- |
| success    | `Boolean`, if the value is true, then the request was successful |


## Endpoints

### /api/stops
Returns a list of [Stop](#stop-s)s

#### /api/stops/near/`<latitude>`/`<longitude>`/`<range>`
Returns a `StopDistanceResult` result which consists of a list of [Stop](#stop-s)s that are within `<range>` meters from
the provided `<latitude>` and `<longitude>`

### /api/trips
Returns a list of [Trip](#trip-t)s

#### /api/trips/`<stop_id>`
Returns the available [Trip](#trip-t)s at the provided [Stop](#stop-s)

## Screenshots

### /api/trips/`<stop_id>`
![/api/trips/<stop_id>](screenshots/1.png)
### /api/stops/
![/api/stops/](screenshots/2.png)
### /api/stops/near/`<latitude>`/`<longitude>`/`<range>`
![/api/stops/near/<latitude>/<longitude>/<range>](screenshots/3.png)