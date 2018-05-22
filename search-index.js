var searchIndex = {};
searchIndex["gtfs_server"] = {"doc":"This is the documentation for `gtfs-server`","items":[[0,"models","gtfs_server","This namespace represents the models used in the API, this part of the documentation should be used as a reference for your classes in your application's wrapper. Please note that some `models` contain a `feed_id` and a `model_id`. These fields are not serialized.",null,null],[0,"agency","gtfs_server::models","Agency related structs and implementations",null,null],[3,"Agency","gtfs_server::models::agency","",null,null],[12,"uid","","",0,null],[12,"id","","",0,null],[12,"name","","",0,null],[12,"url","","",0,null],[12,"timezone","","",0,null],[12,"lang","","",0,null],[12,"phone","","",0,null],[12,"feed_id","","",0,null],[11,"fmt","","",0,{"i":[{"n":"self"},{"n":"formatter"}],"o":{"n":"result"}}],[0,"api","gtfs_server::models","API related models",null,null],[0,"error","gtfs_server::models::api","Error related structs and implementations",null,null],[3,"Error","gtfs_server::models::api::error","",null,null],[12,"code","","",1,null],[12,"message","","",1,null],[11,"fmt","","",1,{"i":[{"n":"self"},{"n":"formatter"}],"o":{"n":"result"}}],[0,"meta","gtfs_server::models::api","Meta related structs and implementations",null,null],[3,"Meta","gtfs_server::models::api::meta","",null,null],[12,"success","","",2,null],[12,"error","","",2,null],[12,"pagination","","",2,null],[11,"fmt","","",2,{"i":[{"n":"self"},{"n":"formatter"}],"o":{"n":"result"}}],[0,"pagination","gtfs_server::models::api","Pagination related structs and implementations",null,null],[3,"Pagination","gtfs_server::models::api::pagination","",null,null],[12,"offset","","",3,null],[12,"limit","","",3,null],[11,"fmt","","",3,{"i":[{"n":"self"},{"n":"formatter"}],"o":{"n":"result"}}],[0,"paginatedvec","gtfs_server::models::api","PaginatedVec model",null,null],[3,"PaginatedVec","gtfs_server::models::api::paginatedvec","",null,null],[12,"vec","","",4,null],[12,"pag","","",4,null],[0,"result","gtfs_server::models::api","Result related structs and implementations",null,null],[3,"Result","gtfs_server::models::api::result","",null,null],[12,"result","","",5,null],[12,"meta","","",5,null],[11,"fmt","","",5,{"i":[{"n":"self"},{"n":"formatter"}],"o":{"n":"result"}}],[0,"resultarray","gtfs_server::models::api","ResultArray related structs and implementations",null,null],[3,"ResultArray","gtfs_server::models::api::resultarray","",null,null],[12,"result","","",6,null],[12,"meta","","",6,null],[11,"fmt","","",6,{"i":[{"n":"self"},{"n":"formatter"}],"o":{"n":"result"}}],[0,"search","gtfs_server::models::api","Search Related Models (Query Parameters)",null,null],[0,"ascdesc","gtfs_server::models::api::search","",null,null],[4,"AscDesc","gtfs_server::models::api::search::ascdesc","",null,null],[13,"ASC","","",7,null],[13,"DESC","","",7,null],[11,"clone","","",7,{"i":[{"n":"self"}],"o":{"n":"ascdesc"}}],[11,"eq","","",7,{"i":[{"n":"self"},{"n":"ascdesc"}],"o":{"n":"bool"}}],[11,"as_str","","",7,{"i":[{"n":"self"}],"o":{"n":"str"}}],[11,"from_form_value","","",7,{"i":[{"n":"rawstr"}],"o":{"n":"result"}}],[0,"route","gtfs_server::models::api::search","",null,null],[3,"RouteSearch","gtfs_server::models::api::search::route","",null,null],[12,"stops_visited","","",8,null],[11,"from_form","","",8,{"i":[{"n":"formitems"},{"n":"bool"}],"o":{"g":["routesearch","error"],"n":"result"}}],[0,"time","gtfs_server::models::api::search","TimeSearch related structs and implementations",null,null],[3,"TimeSearch","gtfs_server::models::api::search::time","",null,null],[12,"date","","",9,null],[12,"service_uid","","",9,null],[12,"monday","","",9,null],[12,"tuesday","","",9,null],[12,"wednesday","","",9,null],[12,"thursday","","",9,null],[12,"friday","","",9,null],[12,"saturday","","",9,null],[12,"sunday","","",9,null],[12,"at_a","","",9,null],[12,"at_b","","",9,null],[12,"dt_a","","",9,null],[12,"dt_b","","",9,null],[12,"trip_id","","",9,null],[12,"pickup_type","","",9,null],[12,"drop_off_type","","",9,null],[12,"stop_sequence","","",9,null],[12,"sort_by","","",9,null],[12,"sort_order","","",9,null],[12,"stop","","",9,null],[12,"route","","",9,null],[12,"trip","","",9,null],[4,"TimeSort","","",null,null],[13,"ArrivalTime","","",10,null],[13,"DepartureTime","","",10,null],[13,"StopSequence","","",10,null],[11,"from_form","","",9,{"i":[{"n":"formitems"},{"n":"bool"}],"o":{"g":["timesearch","error"],"n":"result"}}],[11,"default","","",9,{"o":{"n":"timesearch"}}],[11,"as_str","","",10,{"i":[{"n":"self"}],"o":{"n":"str"}}],[0,"trip","gtfs_server::models::api::search","",null,null],[3,"TripSearch","gtfs_server::models::api::search::trip","",null,null],[12,"stops_visited","","",11,null],[12,"route","","",11,null],[12,"departure_after","","",11,null],[12,"arrival_before","","",11,null],[12,"offset","","",11,null],[12,"per_page","","",11,null],[12,"sort_by","","",11,null],[12,"sort_order","","",11,null],[11,"from_form","","",11,{"i":[{"n":"formitems"},{"n":"bool"}],"o":{"g":["tripsearch","error"],"n":"result"}}],[0,"sort","gtfs_server::models::api","Sort related models",null,null],[0,"tripsort","gtfs_server::models::api::sort","Trip Sort Model",null,null],[4,"TripSort","gtfs_server::models::api::sort::tripsort","",null,null],[13,"ArrivalTime","","",12,null],[13,"DepartureTime","","",12,null],[13,"DirectionId","","",12,null],[13,"ServiceId","","",12,null],[13,"RouteId","","",12,null],[13,"Uid","","",12,null],[13,"None","","",12,null],[11,"clone","","",12,{"i":[{"n":"self"}],"o":{"n":"tripsort"}}],[11,"eq","","",12,{"i":[{"n":"self"},{"n":"tripsort"}],"o":{"n":"bool"}}],[11,"from_form_value","","",12,{"i":[{"n":"rawstr"}],"o":{"n":"result"}}],[0,"stopdistance","gtfs_server::models::api","StopDistance related structs and implementations",null,null],[3,"StopDistance","gtfs_server::models::api::stopdistance","",null,null],[12,"stop","","",13,null],[12,"distance","","",13,null],[11,"fmt","","",13,{"i":[{"n":"self"},{"n":"formatter"}],"o":{"n":"result"}}],[0,"successresult","gtfs_server::models::api","SuccessResult related structs and implementations",null,null],[3,"SuccessResult","gtfs_server::models::api::successresult","",null,null],[12,"success","","",14,null],[11,"fmt","","",14,{"i":[{"n":"self"},{"n":"formatter"}],"o":{"n":"result"}}],[0,"boundingbox","gtfs_server::models","Bouding Box struct and implementation",null,null],[3,"BoundingBox","gtfs_server::models::boundingbox","A Bouding Box is defined (as a parameter) as following:   `p1_lat,p1_lng,p2_lat,p2_lng` where `pn_lat` is the latitude of the n-th point and `pn_lng` is the longitude of the n-th point.  ",null,null],[12,"p1","","",15,null],[12,"p2","","",15,null],[11,"fmt","","",15,{"i":[{"n":"self"},{"n":"formatter"}],"o":{"n":"result"}}],[11,"from_param","","",15,{"i":[{"n":"rawstr"}],"o":{"n":"result"}}],[0,"coordinate","gtfs_server::models","",null,null],[3,"Coordinate","gtfs_server::models::coordinate","",null,null],[12,"lat","","",16,null],[12,"lng","","",16,null],[11,"fmt","","",16,{"i":[{"n":"self"},{"n":"formatter"}],"o":{"n":"result"}}],[0,"csv","gtfs_server::models","This module represents the entities as found in the CSV files. These structs are only used for CSV-parsing. Therefore they are conform to the GTFS reference.",null,null],[0,"agency","gtfs_server::models::csv","",null,null],[3,"AgencyCSV","gtfs_server::models::csv::agency","",null,null],[12,"agency_id","","",17,null],[12,"agency_name","","",17,null],[12,"agency_url","","",17,null],[12,"agency_timezone","","",17,null],[12,"agency_lang","","",17,null],[12,"agency_phone","","",17,null],[12,"agency_fare_url","","",17,null],[12,"agency_email","","",17,null],[11,"fmt","","",17,{"i":[{"n":"self"},{"n":"formatter"}],"o":{"n":"result"}}],[0,"calendar","gtfs_server::models::csv","",null,null],[3,"CalendarCSV","gtfs_server::models::csv::calendar","",null,null],[12,"service_id","","",18,null],[12,"monday","","",18,null],[12,"tuesday","","",18,null],[12,"wednesday","","",18,null],[12,"thursday","","",18,null],[12,"friday","","",18,null],[12,"saturday","","",18,null],[12,"sunday","","",18,null],[12,"start_date","","",18,null],[12,"end_date","","",18,null],[11,"fmt","","",18,{"i":[{"n":"self"},{"n":"formatter"}],"o":{"n":"result"}}],[0,"calendardate","gtfs_server::models::csv","",null,null],[3,"CalendarDateCSV","gtfs_server::models::csv::calendardate","",null,null],[12,"service_id","","",19,null],[12,"date","","",19,null],[12,"exception_type","","",19,null],[11,"fmt","","",19,{"i":[{"n":"self"},{"n":"formatter"}],"o":{"n":"result"}}],[0,"feed","gtfs_server::models::csv","",null,null],[3,"FeedCSV","gtfs_server::models::csv::feed","",null,null],[12,"feed_publisher_name","","",20,null],[12,"feed_publisher_url","","",20,null],[12,"feed_lang","","",20,null],[12,"feed_start_date","","",20,null],[12,"feed_end_date","","",20,null],[12,"feed_version","","",20,null],[11,"fmt","","",20,{"i":[{"n":"self"},{"n":"formatter"}],"o":{"n":"result"}}],[0,"route","gtfs_server::models::csv","",null,null],[3,"RouteCSV","gtfs_server::models::csv::route","",null,null],[12,"route_id","","",21,null],[12,"agency_id","","",21,null],[12,"route_short_name","","",21,null],[12,"route_long_name","","",21,null],[12,"route_desc","","",21,null],[12,"route_type","","",21,null],[12,"route_url","","",21,null],[12,"route_color","","",21,null],[12,"route_text_color","","",21,null],[12,"route_sort_order","","",21,null],[11,"fmt","","",21,{"i":[{"n":"self"},{"n":"formatter"}],"o":{"n":"result"}}],[0,"stop","gtfs_server::models::csv","",null,null],[3,"StopCSV","gtfs_server::models::csv::stop","",null,null],[12,"stop_id","","",22,null],[12,"stop_code","","",22,null],[12,"stop_name","","",22,null],[12,"stop_desc","","",22,null],[12,"stop_lat","","",22,null],[12,"stop_lon","","",22,null],[12,"zone_id","","",22,null],[12,"stop_url","","",22,null],[12,"location_type","","",22,null],[12,"parent_station","","",22,null],[12,"stop_timezone","","",22,null],[12,"wheelchair_boarding","","",22,null],[11,"fmt","","",22,{"i":[{"n":"self"},{"n":"formatter"}],"o":{"n":"result"}}],[0,"stoptime","gtfs_server::models::csv","",null,null],[3,"StopTimeCSV","gtfs_server::models::csv::stoptime","",null,null],[12,"trip_id","","",23,null],[12,"arrival_time","","",23,null],[12,"departure_time","","",23,null],[12,"stop_id","","",23,null],[12,"stop_sequence","","",23,null],[12,"pickup_type","","",23,null],[12,"drop_off_type","","",23,null],[11,"fmt","","",23,{"i":[{"n":"self"},{"n":"formatter"}],"o":{"n":"result"}}],[0,"trip","gtfs_server::models::csv","",null,null],[3,"TripCSV","gtfs_server::models::csv::trip","",null,null],[12,"route_id","","",24,null],[12,"service_id","","",24,null],[12,"trip_id","","",24,null],[12,"trip_headsign","","",24,null],[12,"trip_short_name","","",24,null],[12,"direction_id","","",24,null],[12,"block_id","","",24,null],[12,"shape_id","","",24,null],[12,"wheelchair_accessible","","",24,null],[12,"bikes_allowed","","",24,null],[11,"fmt","","",24,{"i":[{"n":"self"},{"n":"formatter"}],"o":{"n":"result"}}],[0,"dropoff","gtfs_server::models","DropOff related enums and implementations",null,null],[4,"DropOff","gtfs_server::models::dropoff","",null,null],[13,"RegularlyScheduled","","",25,null],[13,"NotAvailable","","",25,null],[13,"MustArrangeWithAgency","","",25,null],[13,"MustCoordinateWithDriver","","",25,null],[11,"fmt","","",25,{"i":[{"n":"self"},{"n":"formatter"}],"o":{"n":"result"}}],[11,"hash","","",25,null],[11,"clone","","",25,{"i":[{"n":"self"}],"o":{"n":"dropoff"}}],[11,"from_string","","",25,{"i":[{"n":"str"}],"o":{"n":"dropoff"}}],[0,"pickup","gtfs_server::models","PickUp related enums and implementations",null,null],[4,"PickUp","gtfs_server::models::pickup","",null,null],[13,"RegularlyScheduled","","",26,null],[13,"NotAvailable","","",26,null],[13,"MustArrangeWithAgency","","",26,null],[13,"MustCoordinateWithDriver","","",26,null],[11,"fmt","","",26,{"i":[{"n":"self"},{"n":"formatter"}],"o":{"n":"result"}}],[11,"hash","","",26,null],[11,"clone","","",26,{"i":[{"n":"self"}],"o":{"n":"pickup"}}],[11,"from_string","","",26,{"i":[{"n":"str"}],"o":{"n":"pickup"}}],[0,"query","gtfs_server::models","Query Representation",null,null],[3,"Query","gtfs_server::models::query","",null,null],[12,"select_v","","",27,null],[12,"from_v","","",27,null],[12,"where_v","","",27,null],[12,"join_v","","",27,null],[12,"order_v","","",27,null],[12,"limit","","",27,null],[12,"offset","","",27,null],[12,"format","","",27,null],[12,"sort_order","","",27,null],[11,"clone","","",27,{"i":[{"n":"self"}],"o":{"n":"query"}}],[11,"format","","",27,{"i":[{"n":"self"}],"o":{"n":"string"}}],[0,"route","gtfs_server::models","Route related structs and implementations",null,null],[3,"Route","gtfs_server::models::route","",null,null],[12,"uid","","",28,null],[12,"id","","",28,null],[12,"agency_id","","",28,null],[12,"short_name","","",28,null],[12,"long_name","","",28,null],[12,"description","","",28,null],[12,"route_type","","",28,null],[12,"feed_id","","",28,null],[11,"fmt","","",28,{"i":[{"n":"self"},{"n":"formatter"}],"o":{"n":"result"}}],[0,"stop","gtfs_server::models","Stop related structs and implementations",null,null],[3,"Stop","gtfs_server::models::stop","",null,null],[12,"uid","","",29,null],[12,"name","","",29,null],[12,"lat","","",29,null],[12,"lng","","",29,null],[12,"location_type","","",29,null],[12,"parent_station","","",29,null],[3,"StopTrip","","",null,null],[12,"stop","","",30,null],[12,"arrival_time","","",30,null],[12,"departure_time","","",30,null],[12,"stop_sequence","","",30,null],[12,"drop_off","","",30,null],[12,"pickup","","",30,null],[11,"fmt","","",29,{"i":[{"n":"self"},{"n":"formatter"}],"o":{"n":"result"}}],[11,"clone","","",29,{"i":[{"n":"self"}],"o":{"n":"stop"}}],[11,"fmt","","",30,{"i":[{"n":"self"},{"n":"formatter"}],"o":{"n":"result"}}],[11,"clone","","",30,{"i":[{"n":"self"}],"o":{"n":"stoptrip"}}],[11,"new","","",29,{"i":[{"n":"string"},{"n":"string"},{"n":"f64"},{"n":"f64"},{"g":["i32"],"n":"option"},{"g":["string"],"n":"option"}],"o":{"n":"stop"}}],[11,"set_id","","",29,{"i":[{"n":"self"},{"n":"string"}]}],[11,"set_feed_id","","",29,{"i":[{"n":"self"},{"n":"string"}]}],[0,"time","gtfs_server::models","Time related structs and implementations",null,null],[3,"Time","gtfs_server::models::time","",null,null],[12,"trip_id","","",31,null],[12,"arrival_time","","",31,null],[12,"departure_time","","",31,null],[12,"stop_id","","",31,null],[12,"stop_sequence","","",31,null],[12,"pickup_type","","",31,null],[12,"drop_off_type","","",31,null],[12,"route_id","","",31,null],[12,"service_days","","",31,null],[12,"service_uid","","",31,null],[12,"start_date","","",31,null],[12,"end_date","","",31,null],[12,"feed_id","","",31,null],[11,"fmt","","",31,{"i":[{"n":"self"},{"n":"formatter"}],"o":{"n":"result"}}],[0,"trip","gtfs_server::models","Trip related structs and implementations",null,null],[3,"Trip","gtfs_server::models::trip","",null,null],[12,"uid","","",32,null],[12,"route_id","","",32,null],[12,"service_id","","",32,null],[12,"headsign","","",32,null],[12,"short_name","","",32,null],[12,"direction_id","","",32,null],[12,"stop_sequence","","",32,null],[11,"fmt","","",32,{"i":[{"n":"self"},{"n":"formatter"}],"o":{"n":"result"}}],[11,"clone","","",32,{"i":[{"n":"self"}],"o":{"n":"trip"}}],[11,"new","","",32,{"i":[{"n":"string"},{"n":"string"},{"n":"string"},{"n":"string"},{"n":"string"},{"n":"i32"}],"o":{"n":"trip"}}],[11,"set_id","","",32,{"i":[{"n":"self"},{"n":"string"}]}],[11,"set_feed_id","","",32,{"i":[{"n":"self"},{"n":"string"}]}],[11,"eq","","",32,{"i":[{"n":"self"},{"n":"trip"}],"o":{"n":"bool"}}],[11,"hash","","",32,{"i":[{"n":"self"},{"n":"h"}]}],[11,"cmp","","",32,{"i":[{"n":"self"},{"n":"self"}],"o":{"n":"ordering"}}],[11,"partial_cmp","","",32,{"i":[{"n":"self"},{"n":"self"}],"o":{"g":["ordering"],"n":"option"}}],[0,"routes","gtfs_server","This model represents all the routes managed by Rocket. Some routes may not be active: you may want to check main.rs for a list of enabled routes.",null,null],[3,"RoutesHandler","gtfs_server::routes","",null,null],[12,"pool","","",33,null],[0,"api","","",null,null],[5,"main","gtfs_server::routes::api","",null,{"o":{"g":["string"],"n":"html"}}],[0,"agency","","`/agency` related routes",null,null],[5,"agency","gtfs_server::routes::api::agency","`/agency` Get the Agencies. Returns a ResultArray<Agency>",null,{"i":[{"g":["routeshandler"],"n":"state"}],"o":{"g":["resultarray"],"n":"json"}}],[5,"agency_by_id","","`/agency/<agency_uid>` Get the the specified Agency by its specified UID. Returns a Result<Agency>",null,{"i":[{"g":["routeshandler"],"n":"state"},{"n":"string"}],"o":{"g":["result"],"n":"json"}}],[5,"get_agency_id","","Returns the UID of the `agency_id` and `feed_id` provided.",null,{"i":[{"g":["string"],"n":"option"},{"n":"string"},{"n":"state"}],"o":{"g":["string"],"n":"option"}}],[7,"static_rocket_route_info_for_agency","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_agency_by_id","","Rocket code generated static route information structure.",null,null],[0,"db","gtfs_server::routes::api","`/db` related routes",null,null],[5,"update","gtfs_server::routes::api::db","`/db/update` Updates the DB schema. This operation should be performed after each update because the DB may have been updated. Returns a SuccessResult",null,{"i":[{"g":["routeshandler"],"n":"state"}],"o":{"g":["successresult"],"n":"json"}}],[5,"version","","`/db/version` Returns the current DB version Returns a Result<i32>",null,{"i":[{"g":["routeshandler"],"n":"state"}],"o":{"g":["result"],"n":"json"}}],[7,"static_rocket_route_info_for_update","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_version","","Rocket code generated static route information structure.",null,null],[0,"import","gtfs_server::routes::api","`/import` related routes",null,null],[5,"url","gtfs_server::routes::api::import","",null,{"i":[{"g":["routeshandler"],"n":"state"},{"n":"string"}],"o":{"g":["successresult"],"n":"json"}}],[5,"fs","","",null,{"i":[{"g":["routeshandler"],"n":"state"},{"n":"string"}],"o":{"g":["successresult"],"n":"json"}}],[5,"agency","","",null,{"i":[{"g":["routeshandler"],"n":"state"},{"n":"string"}],"o":{"g":["successresult"],"n":"json"}}],[5,"stops","","",null,{"i":[{"g":["routeshandler"],"n":"state"},{"n":"string"}],"o":{"g":["successresult"],"n":"json"}}],[5,"times","","",null,{"i":[{"g":["routeshandler"],"n":"state"},{"n":"string"}],"o":{"g":["successresult"],"n":"json"}}],[5,"routes","","",null,{"i":[{"g":["routeshandler"],"n":"state"},{"n":"string"}],"o":{"g":["successresult"],"n":"json"}}],[5,"trips","","",null,{"i":[{"g":["routeshandler"],"n":"state"},{"n":"string"}],"o":{"g":["successresult"],"n":"json"}}],[5,"calendar","","",null,{"i":[{"g":["routeshandler"],"n":"state"},{"n":"string"}],"o":{"g":["successresult"],"n":"json"}}],[7,"static_rocket_route_info_for_url","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_fs","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_agency","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_stops","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_times","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_routes","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_trips","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_calendar","","Rocket code generated static route information structure.",null,null],[0,"routes","gtfs_server::routes::api","`/routes` related routes",null,null],[5,"routes","gtfs_server::routes::api::routes","`/routes`   Returns a ResultArray <Route>",null,{"i":[{"g":["routeshandler"],"n":"state"}],"o":{"g":["resultarray"],"n":"json"}}],[5,"routes_by_query","","`/routes?query` Returns a ResultArray <Route>",null,{"i":[{"g":["routeshandler"],"n":"state"},{"n":"routesearch"}],"o":{"g":["resultarray"],"n":"json"}}],[5,"route_by_id","","`/routes/<route_uid>`   Gets the specified Route by its UID, parametrized as `<route_uid>`.   Returns a Result <Route>",null,{"i":[{"g":["routeshandler"],"n":"state"},{"n":"string"}],"o":{"g":["result"],"n":"json"}}],[5,"route_by_stop_uid","","`/routes/by-stop/<stop_uid>`   Gets the Routes that serve a particular Stop by its UID, parametrized as `<stop_uid>`.   Returns a Result <Route>",null,{"i":[{"g":["routeshandler"],"n":"state"},{"n":"string"}],"o":{"g":["resultarray"],"n":"json"}}],[5,"route_by_bbox","","`/routes/by-bbox/<bbox>/`   Gets the Routes that serve a particular Stop by its UID, parametrized as `<stop_uid>`.   Returns a Result <Route>",null,{"i":[{"g":["routeshandler"],"n":"state"},{"n":"boundingbox"}],"o":{"g":["resultarray"],"n":"json"}}],[7,"static_rocket_route_info_for_routes","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_routes_by_query","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_route_by_id","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_route_by_stop_uid","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_route_by_bbox","","Rocket code generated static route information structure.",null,null],[0,"stops","gtfs_server::routes::api","`/stops` related routes",null,null],[5,"stops","gtfs_server::routes::api::stops","`/stops`   Returns a ResultArray<Stop>",null,{"i":[{"g":["routeshandler"],"n":"state"}],"o":{"g":["resultarray"],"n":"json"}}],[5,"stops_by_id","","`/stops/<stop_id>`   Gets a single Stop from its `stop_id`.   Returns a Result<Stop>",null,{"i":[{"g":["routeshandler"],"n":"state"},{"n":"string"}],"o":{"g":["result"],"n":"json"}}],[5,"stops_by_trip","","`/stops/by-trip/<trip_id>`   get the Stops visited by a Trip uid.   Returns a ResultArray<Stop>",null,{"i":[{"g":["routeshandler"],"n":"state"},{"n":"string"}],"o":{"g":["resultarray"],"n":"json"}}],[5,"stops_near_default","","`/stops/near/<lat>/<lng>`   Gets an array of StopDistances, within 100.0 meters from `<lat>`,`<lng>` - nearest first.   Returns a ResultArray <StopDistance>",null,{"i":[{"g":["routeshandler"],"n":"state"},{"n":"f32"},{"n":"f32"}],"o":{"g":["resultarray"],"n":"json"}}],[5,"stops_near","","`/stops/near/<lat>/<lng>/<meters>`   Gets an array of StopDistances, within `<meters>` meters from `<lat>`,`<lng>` nearest first, of Stops near the provided coordinate.   Returns a ResultArray <StopDistance>",null,{"i":[{"g":["routeshandler"],"n":"state"},{"n":"f32"},{"n":"f32"},{"n":"f64"}],"o":{"g":["resultarray"],"n":"json"}}],[5,"stops_in_bbox","","`/stops/in/<bbox>`   Gets an array of Stops, inside a BoudingBox defined by two points (P1 and P2).",null,{"i":[{"g":["routeshandler"],"n":"state"},{"n":"boundingbox"}],"o":{"g":["resultarray"],"n":"json"}}],[5,"stops_in_bbox_radius","","`/stops/in/<bbox>/<meters>`   Gets an array of Stops, inside a Bouding Box defined by two circles of a radius `<meters>` meters with centers in P1 and P2.",null,{"i":[{"g":["routeshandler"],"n":"state"},{"n":"boundingbox"},{"n":"f64"}],"o":{"g":["resultarray"],"n":"json"}}],[5,"stops_latlng_test","","",null,{"i":[{"n":"f64"},{"n":"f64"},{"n":"f64"},{"n":"f64"},{"n":"f64"},{"n":"f64"}],"o":{"g":["string"],"n":"html"}}],[5,"stops_latlng_test_zoom","","",null,{"i":[{"n":"f64"},{"n":"f64"},{"n":"f64"},{"n":"f64"},{"n":"f64"},{"n":"f64"},{"n":"i32"}],"o":{"g":["string"],"n":"html"}}],[7,"static_rocket_route_info_for_stops","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_stops_by_id","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_stops_by_trip","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_stops_near_default","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_stops_near","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_stops_in_bbox","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_stops_in_bbox_radius","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_stops_latlng_test","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_stops_latlng_test_zoom","","Rocket code generated static route information structure.",null,null],[0,"times","gtfs_server::routes::api","`/times` related routes",null,null],[5,"times_query","gtfs_server::routes::api::times","",null,{"i":[{"g":["routeshandler"],"n":"state"},{"n":"timesearch"}],"o":{"g":["resultarray"],"n":"json"}}],[5,"times_by_trip","","`/times/by-trip/<trip_uid>`   Gets the Times associated to the specified Trip UID, parametrized as `<trip_id>`.   Returns a ResultArray <Time>",null,{"i":[{"g":["routeshandler"],"n":"state"},{"n":"string"}],"o":{"g":["resultarray"],"n":"json"}}],[5,"times_stop_query","","`/times/by-stop/<stop_id>?<time_search>`   Gets the Times associated to the specified Stop UID, parametrized as `<stop_id>`.   The results can be filtered with `<time_search>` parameters (check TimeSearch)   Returns a ResultArray <Time>",null,{"i":[{"g":["routeshandler"],"n":"state"},{"n":"string"},{"n":"timesearch"}],"o":{"g":["resultarray"],"n":"json"}}],[5,"times_stop","","`/times/by-stop/<stop_id>`   Gets the Times associated to the specified Stop UID, parametrized as `<stop_id>`.   Returns a ResultArray <Time>",null,{"i":[{"g":["routeshandler"],"n":"state"},{"n":"string"}],"o":{"g":["resultarray"],"n":"json"}}],[5,"get_times_by_trip","","",null,{"i":[{"n":"string"},{"n":"pool"}],"o":{"g":["time"],"n":"vec"}}],[7,"static_rocket_route_info_for_times_query","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_times_by_trip","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_times_stop_query","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_times_stop","","Rocket code generated static route information structure.",null,null],[0,"trips","gtfs_server::routes::api","`/trips` related routes",null,null],[5,"trips","gtfs_server::routes::api::trips","`/trips/`, returns a list of Trips. Returns a ResultArray <Trip>",null,{"i":[{"g":["routeshandler"],"n":"state"}],"o":{"g":["resultarray"],"n":"json"}}],[5,"trips_by_query","","`/trips/?query`, returns a list of Trips filtered with a TripSearch query. Returns a ResultArray <Trip>",null,{"i":[{"g":["routeshandler"],"n":"state"},{"n":"tripsearch"}],"o":{"g":["resultarray"],"n":"json"}}],[5,"trips_stopid","","`/trips/by-stop/<stop_id>`, returns the Trips associated to the specified Stop UID, parametrized as `<stop_id>`. Returns a ResultArray <Trip>",null,{"i":[{"g":["routeshandler"],"n":"state"},{"n":"string"}],"o":{"g":["resultarray"],"n":"json"}}],[5,"trip","","`/trips/<trip_id>`, returns the Trips associated to the specified Trip UID, parametrized as `<trip_id>`. Returns a Result <Trip>",null,{"i":[{"g":["routeshandler"],"n":"state"},{"n":"string"}],"o":{"g":["result"],"n":"json"}}],[5,"trips_by_route","","`/trips/by-route/<route_uid>`, returns the Trips associated to the specified Route UID, parametrized as `<route_uid>`. Returns a Result <Trip>",null,{"i":[{"g":["routeshandler"],"n":"state"},{"n":"string"}],"o":{"g":["resultarray"],"n":"json"}}],[5,"trips_by_bbox","","`/trips/in/<bbox>`, returns the Trips contained in a Bounding Box. Returns a ResultArray <Trip>",null,{"i":[{"g":["routeshandler"],"n":"state"},{"n":"boundingbox"}],"o":{"g":["resultarray"],"n":"json"}}],[5,"trips_by_bbox_query","","`/trips/in/<bbox>?<query>`, returns the Trips contained in a Bounding Box, filtered w/ the TripSearch query. Returns a ResultArray <Trip>",null,{"i":[{"g":["routeshandler"],"n":"state"},{"n":"boundingbox"},{"n":"tripsearch"}],"o":{"g":["resultarray"],"n":"json"}}],[7,"static_rocket_route_info_for_trips","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_trips_by_query","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_trips_stopid","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_trip","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_trips_by_route","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_trips_by_bbox","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_trips_by_bbox_query","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_main","gtfs_server::routes::api","Rocket code generated static route information structure.",null,null],[0,"ui","gtfs_server::routes","UI Routes",null,null],[0,"import","gtfs_server::routes::ui","`/import` UI related routes",null,null],[5,"main","gtfs_server::routes::ui::import","",null,{"o":{"g":["string"],"n":"html"}}],[7,"static_rocket_route_info_for_main","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_static_css","gtfs_server","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_static_js","","Rocket code generated static route information structure.",null,null]],"paths":[[3,"Agency"],[3,"Error"],[3,"Meta"],[3,"Pagination"],[3,"PaginatedVec"],[3,"Result"],[3,"ResultArray"],[4,"AscDesc"],[3,"RouteSearch"],[3,"TimeSearch"],[4,"TimeSort"],[3,"TripSearch"],[4,"TripSort"],[3,"StopDistance"],[3,"SuccessResult"],[3,"BoundingBox"],[3,"Coordinate"],[3,"AgencyCSV"],[3,"CalendarCSV"],[3,"CalendarDateCSV"],[3,"FeedCSV"],[3,"RouteCSV"],[3,"StopCSV"],[3,"StopTimeCSV"],[3,"TripCSV"],[4,"DropOff"],[4,"PickUp"],[3,"Query"],[3,"Route"],[3,"Stop"],[3,"StopTrip"],[3,"Time"],[3,"Trip"],[3,"RoutesHandler"]]};
initSearch(searchIndex);
