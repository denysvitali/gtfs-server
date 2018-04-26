SELECT 
    trip.uid,
    route.uid,
    calendar.uid,
    trip.trip_id,
    trip.headsign,
    trip.short_name,
    trip.direction_id,
    trip.feed_id 
    (   SELECT 
        stop.uid, 
        stop.name, 
        ST_Y(position::geometry) as lat,
        ST_X(position::geometry) as lng,
        stop."type", 
        (SELECT stop.uid FROM stop as s WHERE s.id = stop.parent_stop AND s.feed_id = stop.feed_id) as parent_stop, 
        stop_time.stop_sequence,  
        stop_time.drop_off_type,
        stop_time.pickup_type,
        stop_time.arrival_time, 
        stop_time.departure_time 
        FROM stop_time, stop
        WHERE stop.id = stop_time.stop_id
        AND stop_time.trip_id = (SELECT trip.trip_id FROM trip WHERE trip.uid = $1 AND trip.feed_id = stop.feed_id) 
        AND stop.feed_id = stop_time.feed_id 
        ORDER BY stop_sequence ASC
    ) as stop_trip 
    FROM trip, route, calendar 
    WHERE trip.uid IN ( 
        SELECT trip.uid
        FROM trip 
        WHERE EXISTS ( 
            SELECT 1 
            FROM stop AS s 
            INNER JOIN stop_time AS st 
            ON s.id = st.stop_id AND s.feed_id = st.feed_id 
            WHERE ST_Within(s.position::geometry, 
                        ST_MakeEnvelope($1, $2, $3, $4, 4326)) 
            AND st.trip_id = trip.trip_id AND trip.feed_id = st.feed_id 
        ) 
        LIMIT 50 
    ) 
    AND 
    route.feed_id = trip.feed_id AND 
    calendar.feed_id = trip.feed_id AND 
    route.id = trip.route_id AND 
    calendar.service_id = trip.service_id



    ----

SELECT * FROM 
    (SELECT 
        trip.uid as tuid,
        route.uid,
        calendar.uid,
        trip.trip_id,
        trip.headsign,
        trip.short_name,
        trip.direction_id,
        trip.feed_id  
        FROM trip, route, calendar 
        WHERE trip.uid IN ( 
            SELECT trip.uid
            FROM trip 
            WHERE EXISTS ( 
                SELECT 1 
                FROM stop AS s 
                INNER JOIN stop_time AS st 
                ON s.id = st.stop_id AND s.feed_id = st.feed_id 
                WHERE ST_Within(s.position::geometry, 
                            ST_MakeEnvelope(8.974125,46.01946,8.967738,46.023113, 4326))
                AND st.trip_id = trip.trip_id AND trip.feed_id = st.feed_id 
            ) 
            LIMIT 50 
        ) 
        AND 
        route.feed_id = trip.feed_id AND 
        calendar.feed_id = trip.feed_id AND 
        route.id = trip.route_id AND 
        calendar.service_id = trip.service_id
    ) as trip
 INNER JOIN stop_time ON stop_time.trip_id = trip.trip_id AND stop_time.feed_id = trip.feed_id
 ORDER BY (trip.tuid, stop_sequence)

---

SELECT
	tuid,
	ruid,
	cuid,
	ths,
	tsn,
	td,
	tfid,
	stop.uid as suid,
	stop.id as sid,
	stop."name" as sname,
	ST_Y(stop.position::geometry) as slat,
    ST_X(stop.position::geometry) as slng,
    stop."type" as st,
    pstop.uid,
    stop_time.arrival_time as st_at,
	stop_time.departure_time as st_dt,
	stop_time.stop_sequence as st_ss,
	stop_time.drop_off_type as st_do,
	stop_time.pickup_type as st_pu
FROM
(SELECT
    trip.uid as tuid,
    route.uid as ruid,
    calendar.uid as cuid,
    trip.trip_id as tid,
    trip.headsign as ths,
    trip.short_name as tsn,
    trip.direction_id as td,
    trip.feed_id as tfid
    FROM trip, route, calendar
    WHERE trip.uid IN (
        SELECT trip.uid
        FROM trip
        WHERE EXISTS (
            SELECT 1
            FROM stop AS s
            INNER JOIN stop_time AS st
            ON s.id = st.stop_id AND s.feed_id = st.feed_id
            WHERE ST_Within(s.position::geometry,
                        ST_MakeEnvelope(8.974125,46.01946,8.967738,46.023113, 4326))
            AND st.trip_id = trip.trip_id AND trip.feed_id = st.feed_id
        )
        LIMIT 50
    )
    AND
    route.feed_id = trip.feed_id AND
    calendar.feed_id = trip.feed_id AND
    route.id = trip.route_id AND
    calendar.service_id = trip.service_id
 ) as trip
 INNER JOIN stop_time ON stop_time.trip_id = tid AND stop_time.feed_id = tfid
 INNER JOIN stop ON stop_time.stop_id = stop.id AND stop_time.feed_id = stop.feed_id
 LEFT JOIN stop as pstop ON stop.id = stop.parent_stop AND stop.feed_id = tfid
 ORDER BY (trip.tuid, stop_sequence)