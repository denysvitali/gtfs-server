CREATE TABLE public.agency (
	uid varchar(255) NOT NULL,
	id varchar(255) NOT NULL,
	"name" varchar(255) NOT NULL,
	url varchar(512) NULL,
	timezone varchar(255) NULL,
	lang varchar(20) NULL,
	phone varchar(255) NULL,
	feed_id varchar(64) NOT NULL,
	fare_url varchar(512) NULL,
	email varchar(255) NULL,
	CONSTRAINT agency_pkey PRIMARY KEY (id, feed_id)
)
WITH (
	OIDS=FALSE
) ;

CREATE TABLE public.calendar (
	uid varchar(255) NOT NULL,
	service_id varchar(255) NOT NULL,
	monday bool NULL,
	tuesday bool NULL,
	wednesday bool NULL,
	thursday bool NULL,
	friday bool NULL,
	saturday bool NULL,
	sunday bool NULL,
	start_date date NULL,
	end_date date NULL,
	feed_id varchar(64) NOT NULL,
	CONSTRAINT calendar_pkey PRIMARY KEY (uid),
	CONSTRAINT calendar_service_id_feed_id_key UNIQUE (service_id, feed_id)
)
WITH (
	OIDS=FALSE
) ;
CREATE UNIQUE INDEX calendar_service_id_idx ON public.calendar USING btree (service_id, feed_id) ;

CREATE TABLE public.calendar_date (
	uid varchar(255) NOT NULL,
	service_id varchar(255) NOT NULL,
	"date" date NOT NULL,
	exception_type int4 NOT NULL,
	feed_id varchar(64) NOT NULL,
	CONSTRAINT calendar_date_pkey PRIMARY KEY (uid),
	CONSTRAINT calendar_date_service_id_date_feed_id_key UNIQUE (service_id, date, feed_id)
)
WITH (
	OIDS=FALSE
) ;

CREATE TABLE public.feed (
	id varchar(64) NOT NULL,
	publisher_name varchar(255) NULL,
	publisher_url varchar(255) NULL,
	lang varchar(20) NULL,
	start_date date NOT NULL,
	end_date date NOT NULL,
	"version" varchar(255) NULL,
	CONSTRAINT feed_pkey PRIMARY KEY (id)
)
WITH (
	OIDS=FALSE
) ;

CREATE TABLE public.route (
	uid varchar(255) NOT NULL,
	id varchar(255) NOT NULL,
	agency varchar(255) NULL,
	short_name varchar(255) NOT NULL,
	long_name varchar(255) NOT NULL,
	description varchar(255) NULL,
	"type" int4 NULL,
	feed_id varchar(64) NOT NULL,
	url varchar(255) NULL,
	color bpchar(6) NULL,
	text_color bpchar(6) NULL,
	sort_order int4 NULL,
	CONSTRAINT route_idx UNIQUE (id, feed_id),
	CONSTRAINT route_pk PRIMARY KEY (uid),
	CONSTRAINT route_agency_fk FOREIGN KEY (agency, feed_id) REFERENCES agency(id, feed_id)
)
WITH (
	OIDS=FALSE
) ;
CREATE UNIQUE INDEX route_id_idx ON public.route USING btree (id, feed_id) ;
CREATE UNIQUE INDEX route_uid_idx ON public.route USING btree (uid) ;

CREATE TABLE public.settings (
	"key" varchar(255) NOT NULL,
	value varchar(255) NULL,
	CONSTRAINT settings_pkey PRIMARY KEY (key)
)
WITH (
	OIDS=FALSE
) ;

CREATE TABLE public.stop (
	uid varchar(255) NOT NULL,
	id varchar(255) NOT NULL,
	"name" varchar(255) NOT NULL,
	"position" geography NULL,
	"type" int4 NULL,
	parent_stop varchar(255) NULL,
	feed_id varchar(64) NOT NULL,
	description varchar(512) NULL,
	code varchar(255) NULL,
	zone_id int4 NULL,
	url varchar(512) NULL,
	timezone varchar(255) NULL,
	wheelchair_boarding int4 NULL,
	CONSTRAINT stop_id_feed_id_key UNIQUE (id, feed_id),
	CONSTRAINT stop_pkey PRIMARY KEY (uid)
)
WITH (
	OIDS=FALSE
) ;
CREATE INDEX stop_geom ON public.stop USING gist ("position") ;
CREATE UNIQUE INDEX stop_id_idx ON public.stop USING btree (id, feed_id) ;

CREATE TABLE public.stop_time (
	trip_id varchar(255) NOT NULL,
	arrival_time time NOT NULL,
	departure_time time NOT NULL,
	stop_id varchar(255) NOT NULL,
	stop_sequence int4 NOT NULL,
	pickup_type int4 NULL,
	drop_off_type int4 NULL,
	feed_id varchar(64) NOT NULL,
	stop_headsign varchar(255) NULL,
	shape_dist_traveled float4 NULL,
	timepoint int4 NULL,
	CONSTRAINT stop_time_pkey PRIMARY KEY (trip_id, stop_id, stop_sequence, feed_id),
	CONSTRAINT stop_time_stop_fk FOREIGN KEY (stop_id, feed_id) REFERENCES stop(id, feed_id)
)
WITH (
	OIDS=FALSE
) ;
CREATE INDEX "stop_time_sid-fid" ON public.stop_time USING btree (stop_id, feed_id) ;
CREATE INDEX stop_time_stop_id_idx ON public.stop_time USING btree (stop_id) ;
CREATE INDEX stop_time_trip_id_feed_idx ON public.stop_time USING btree (trip_id, stop_id, feed_id) ;
CREATE INDEX stop_time_trip_id_idx ON public.stop_time USING btree (trip_id, feed_id) ;

CREATE TABLE public.transfer (
	from_sid varchar(255) NOT NULL,
	to_sid varchar(255) NOT NULL,
	transfer_type int4 NULL,
	min_transfer_time int4 NULL,
	feed_id varchar(64) NOT NULL,
	CONSTRAINT transfer_pkey PRIMARY KEY (from_sid, to_sid, feed_id)
)
WITH (
	OIDS=FALSE
) ;

CREATE TABLE public.trip (
	uid varchar(255) NOT NULL,
	route_id varchar(255) NOT NULL,
	service_id varchar(255) NOT NULL,
	trip_id varchar(255) NOT NULL,
	headsign varchar(255) NULL,
	short_name varchar(255) NULL,
	direction_id int4 NULL,
	feed_id varchar(64) NOT NULL,
	block_id varchar(255) NULL,
	shape_id varchar(255) NULL,
	wheelchair_accessible int4 NULL,
	bikes_allowed int4 NULL,
	CONSTRAINT trip_pkey PRIMARY KEY (uid)
)
WITH (
	OIDS=FALSE
) ;
CREATE INDEX trip_feed_id_idx ON public.trip USING btree (feed_id, trip_id) ;
CREATE INDEX trip_idx1 ON public.trip USING btree (feed_id, route_id, trip_id, service_id) ;
CREATE INDEX trip_route_id_idx ON public.trip USING btree (route_id, feed_id) ;
CREATE INDEX trip_trip_id_idx ON public.trip USING btree (trip_id, route_id, feed_id) ;
