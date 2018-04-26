-- 45.818,5.9559,47.8084,10.4921
EXPLAIN (ANALYZE, COSTS, VERBOSE, BUFFERS, FORMAT JSON) SELECT t.uid FROM trip as t 
INNER JOIN stop_time as st ON st.trip_id = t.trip_id AND st.feed_id = t.feed_id
INNER JOIN stop as s ON s.id = st.stop_id AND s.feed_id = st.feed_id
WHERE
s.feed_id = t.feed_id AND
st.stop_id = s.id AND
st.feed_id = s.feed_id AND
st.trip_id = t.trip_id AND
t.feed_id = st.feed_id AND
ST_Contains( 
    ST_MakeEnvelope(
        '5.9559',
        '45.818',
        '10.4921',
        '47.8084',
        4326),
    ST_Transform(s.position::geometry,4326)
)
GROUP BY t.uid

-- Fixed:
SELECT trip.uid
  FROM trip
 WHERE EXISTS (
    SELECT 1
    FROM stop AS s
    INNER JOIN stop_time AS st
    ON s.id = st.stop_id AND s.feed_id = st.feed_id
    WHERE ST_Within(s.position::geometry,
                ST_MakeEnvelope(5.9559, 45.818, 10.4921, 47.8084, 4326))
    AND st.trip_id = trip.trip_id AND trip.feed_id = st.feed_id
)

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
    pstop.uid
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