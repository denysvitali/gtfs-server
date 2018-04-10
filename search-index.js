var searchIndex = {};
searchIndex["gtfs_server"] = {"doc":"This is the documentation for `gtfs-server`","items":[[0,"models","gtfs_server","This namespace represents the models used in the API, this part of the documentation should be used as a reference for your classes in your application's wrapper. Please note that some `models` contain a `feed_id` and a `model_id`. These fields are not serialized.",null,null],[0,"agency","gtfs_server::models","Agency related structs and implementations",null,null],[3,"Agency","gtfs_server::models::agency","",null,null],[12,"uid","","",0,null],[12,"id","","",0,null],[12,"name","","",0,null],[12,"url","","",0,null],[12,"timezone","","",0,null],[12,"lang","","",0,null],[12,"phone","","",0,null],[12,"feed_id","","",0,null],[11,"fmt","","",0,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[0,"api","gtfs_server::models","API related models",null,null],[0,"error","gtfs_server::models::api","Error related structs and implementations",null,null],[3,"Error","gtfs_server::models::api::error","",null,null],[12,"code","","",1,null],[12,"message","","",1,null],[11,"fmt","","",1,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[0,"meta","gtfs_server::models::api","Meta related structs and implementations",null,null],[3,"Meta","gtfs_server::models::api::meta","",null,null],[12,"success","","",2,null],[12,"error","","",2,null],[11,"fmt","","",2,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[0,"result","gtfs_server::models::api","Result related structs and implementations",null,null],[3,"Result","gtfs_server::models::api::result","",null,null],[12,"result","","",3,null],[12,"meta","","",3,null],[11,"fmt","","",3,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[0,"resultarray","gtfs_server::models::api","ResultArray related structs and implementations",null,null],[3,"ResultArray","gtfs_server::models::api::resultarray","",null,null],[12,"result","","",4,null],[12,"meta","","",4,null],[11,"fmt","","",4,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[0,"search","gtfs_server::models::api","Search Related Models (Query Parameters)",null,null],[0,"ascdesc","gtfs_server::models::api::search","",null,null],[4,"AscDesc","gtfs_server::models::api::search::ascdesc","",null,null],[13,"ASC","","",5,null],[13,"DESC","","",5,null],[11,"as_str","","",5,{"inputs":[{"name":"self"}],"output":{"name":"str"}}],[0,"route","gtfs_server::models::api::search","",null,null],[3,"RouteSearch","gtfs_server::models::api::search::route","",null,null],[12,"stops_visited","","",6,null],[11,"from_form","","",6,{"inputs":[{"name":"formitems"},{"name":"bool"}],"output":{"generics":["routesearch","error"],"name":"result"}}],[0,"time","gtfs_server::models::api::search","TimeSearch related structs and implementations",null,null],[3,"TimeSearch","gtfs_server::models::api::search::time","",null,null],[12,"date","","",7,null],[12,"service_uid","","",7,null],[12,"monday","","",7,null],[12,"tuesday","","",7,null],[12,"wednesday","","",7,null],[12,"thursday","","",7,null],[12,"friday","","",7,null],[12,"saturday","","",7,null],[12,"sunday","","",7,null],[12,"at_a","","",7,null],[12,"at_b","","",7,null],[12,"dt_a","","",7,null],[12,"dt_b","","",7,null],[12,"trip_id","","",7,null],[12,"pickup_type","","",7,null],[12,"drop_off_type","","",7,null],[12,"stop_sequence","","",7,null],[12,"sort_by","","",7,null],[12,"sort_order","","",7,null],[12,"stop","","",7,null],[12,"route","","",7,null],[12,"trip","","",7,null],[4,"TimeSort","","",null,null],[13,"arrival_time","","",8,null],[13,"departure_time","","",8,null],[13,"stop_sequence","","",8,null],[11,"from_form","","",7,{"inputs":[{"name":"formitems"},{"name":"bool"}],"output":{"generics":["timesearch","error"],"name":"result"}}],[11,"default","","",7,{"inputs":[],"output":{"name":"timesearch"}}],[11,"as_str","","",8,{"inputs":[{"name":"self"}],"output":{"name":"str"}}],[0,"trip","gtfs_server::models::api::search","",null,null],[3,"TripSearch","gtfs_server::models::api::search::trip","",null,null],[12,"stops_visited","","",9,null],[12,"route","","",9,null],[11,"from_form","","",9,{"inputs":[{"name":"formitems"},{"name":"bool"}],"output":{"generics":["tripsearch","error"],"name":"result"}}],[0,"stopdistance","gtfs_server::models::api","StopDistance related structs and implementations",null,null],[3,"StopDistance","gtfs_server::models::api::stopdistance","",null,null],[12,"stop","","",10,null],[12,"distance","","",10,null],[11,"fmt","","",10,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[0,"successresult","gtfs_server::models::api","SuccessResult related structs and implementations",null,null],[3,"SuccessResult","gtfs_server::models::api::successresult","",null,null],[12,"success","","",11,null],[11,"fmt","","",11,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[0,"csv","gtfs_server::models","This module represents the entities as found in the CSV files. These structs are only used for CSV-parsing. Therefore they are conform to the GTFS reference.",null,null],[0,"agency","gtfs_server::models::csv","",null,null],[3,"AgencyCSV","gtfs_server::models::csv::agency","",null,null],[12,"agency_id","","",12,null],[12,"agency_name","","",12,null],[12,"agency_url","","",12,null],[12,"agency_timezone","","",12,null],[12,"agency_lang","","",12,null],[12,"agency_phone","","",12,null],[11,"fmt","","",12,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[0,"calendar","gtfs_server::models::csv","",null,null],[3,"CalendarCSV","gtfs_server::models::csv::calendar","",null,null],[12,"service_id","","",13,null],[12,"monday","","",13,null],[12,"tuesday","","",13,null],[12,"wednesday","","",13,null],[12,"thursday","","",13,null],[12,"friday","","",13,null],[12,"saturday","","",13,null],[12,"sunday","","",13,null],[12,"start_date","","",13,null],[12,"end_date","","",13,null],[11,"fmt","","",13,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[0,"feed","gtfs_server::models::csv","",null,null],[3,"FeedCSV","gtfs_server::models::csv::feed","",null,null],[12,"feed_publisher_name","","",14,null],[12,"feed_publisher_url","","",14,null],[12,"feed_lang","","",14,null],[12,"feed_start_date","","",14,null],[12,"feed_end_date","","",14,null],[12,"feed_version","","",14,null],[11,"fmt","","",14,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[0,"route","gtfs_server::models::csv","",null,null],[3,"RouteCSV","gtfs_server::models::csv::route","",null,null],[12,"route_id","","",15,null],[12,"agency_id","","",15,null],[12,"route_short_name","","",15,null],[12,"route_long_name","","",15,null],[12,"route_desc","","",15,null],[12,"route_type","","",15,null],[11,"fmt","","",15,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[0,"stop","gtfs_server::models::csv","",null,null],[3,"StopCSV","gtfs_server::models::csv::stop","",null,null],[12,"stop_id","","",16,null],[12,"stop_name","","",16,null],[12,"stop_lat","","",16,null],[12,"stop_lon","","",16,null],[12,"location_type","","",16,null],[12,"parent_station","","",16,null],[11,"fmt","","",16,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[0,"stoptime","gtfs_server::models::csv","",null,null],[3,"StopTimeCSV","gtfs_server::models::csv::stoptime","",null,null],[12,"trip_id","","",17,null],[12,"arrival_time","","",17,null],[12,"departure_time","","",17,null],[12,"stop_id","","",17,null],[12,"stop_sequence","","",17,null],[12,"pickup_type","","",17,null],[12,"drop_off_type","","",17,null],[11,"fmt","","",17,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[0,"trip","gtfs_server::models::csv","",null,null],[3,"TripCSV","gtfs_server::models::csv::trip","",null,null],[12,"route_id","","",18,null],[12,"service_id","","",18,null],[12,"trip_id","","",18,null],[12,"trip_headsign","","",18,null],[12,"trip_short_name","","",18,null],[12,"direction_id","","",18,null],[11,"fmt","","",18,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[0,"dropoff","gtfs_server::models","DropOff related enums and implementations",null,null],[4,"DropOff","gtfs_server::models::dropoff","",null,null],[13,"RegularlyScheduled","","",19,null],[13,"NotAvailable","","",19,null],[13,"MustArrangeWithAgency","","",19,null],[13,"MustCoordinateWithDriver","","",19,null],[11,"fmt","","",19,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"from_string","","",19,{"inputs":[{"name":"str"}],"output":{"name":"dropoff"}}],[0,"pickup","gtfs_server::models","PickUp related enums and implementations",null,null],[4,"PickUp","gtfs_server::models::pickup","",null,null],[13,"RegularlyScheduled","","",20,null],[13,"NoPickupAvailable","","",20,null],[13,"MustArrangeWithAgency","","",20,null],[13,"MustCoordinateWithDriver","","",20,null],[11,"fmt","","",20,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"from_string","","",20,{"inputs":[{"name":"str"}],"output":{"name":"pickup"}}],[0,"route","gtfs_server::models","Route related structs and implementations",null,null],[3,"Route","gtfs_server::models::route","",null,null],[12,"uid","","",21,null],[12,"id","","",21,null],[12,"agency_id","","",21,null],[12,"short_name","","",21,null],[12,"long_name","","",21,null],[12,"description","","",21,null],[12,"route_type","","",21,null],[12,"feed_id","","",21,null],[11,"fmt","","",21,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[0,"stop","gtfs_server::models","Stop related structs and implementations",null,null],[3,"Stop","gtfs_server::models::stop","",null,null],[12,"uid","","",22,null],[12,"name","","",22,null],[12,"lat","","",22,null],[12,"lng","","",22,null],[12,"location_type","","",22,null],[12,"parent_station","","",22,null],[3,"StopTrip","","",null,null],[12,"stop","","",23,null],[12,"arrival_time","","",23,null],[12,"departure_time","","",23,null],[12,"stop_sequence","","",23,null],[12,"drop_off","","",23,null],[12,"pickup","","",23,null],[11,"fmt","","",22,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"fmt","","",23,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"new","","",22,{"inputs":[{"name":"string"},{"name":"string"},{"name":"f64"},{"name":"f64"},{"name":"i32"},{"generics":["string"],"name":"option"}],"output":{"name":"stop"}}],[11,"set_id","","",22,{"inputs":[{"name":"self"},{"name":"string"}],"output":null}],[11,"set_feed_id","","",22,{"inputs":[{"name":"self"},{"name":"string"}],"output":null}],[0,"time","gtfs_server::models","Time related structs and implementations",null,null],[3,"Time","gtfs_server::models::time","",null,null],[12,"trip_id","","",24,null],[12,"arrival_time","","",24,null],[12,"departure_time","","",24,null],[12,"stop_id","","",24,null],[12,"stop_sequence","","",24,null],[12,"pickup_type","","",24,null],[12,"drop_off_type","","",24,null],[12,"route_id","","",24,null],[12,"service_days","","",24,null],[12,"service_uid","","",24,null],[12,"start_date","","",24,null],[12,"end_date","","",24,null],[12,"feed_id","","",24,null],[11,"fmt","","",24,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[0,"trip","gtfs_server::models","Trip related structs and implementations",null,null],[3,"Trip","gtfs_server::models::trip","",null,null],[12,"uid","","",25,null],[12,"route_id","","",25,null],[12,"service_id","","",25,null],[12,"headsign","","",25,null],[12,"short_name","","",25,null],[12,"direction_id","","",25,null],[12,"stop_sequence","","",25,null],[11,"fmt","","",25,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"new","","",25,{"inputs":[{"name":"string"},{"name":"string"},{"name":"string"},{"name":"string"},{"name":"string"},{"name":"i32"}],"output":{"name":"trip"}}],[11,"set_id","","",25,{"inputs":[{"name":"self"},{"name":"string"}],"output":null}],[11,"set_feed_id","","",25,{"inputs":[{"name":"self"},{"name":"string"}],"output":null}],[0,"routes","gtfs_server","This model represents all the routes managed by Rocket. Some routes may not be active: you may want to check main.rs for a list of enabled routes.",null,null],[3,"RoutesHandler","gtfs_server::routes","",null,null],[12,"pool","","",26,null],[0,"api","","",null,null],[5,"main","gtfs_server::routes::api","",null,{"inputs":[],"output":{"generics":["string"],"name":"html"}}],[0,"agency","","`/agency` related routes",null,null],[5,"agency_by_id","gtfs_server::routes::api::agency","`/agency/<agency_uid>`   Get the the specified Agency by its specified UID.   Returns a Result<Agency>",null,{"inputs":[{"generics":["routeshandler"],"name":"state"},{"name":"string"}],"output":{"generics":["result"],"name":"json"}}],[5,"get_agency_id","","Returns the UID of the `agency_id` and `feed_id` provided.",null,{"inputs":[{"generics":["string"],"name":"option"},{"name":"string"},{"name":"state"}],"output":{"generics":["string"],"name":"option"}}],[7,"static_rocket_route_info_for_agency_by_id","","Rocket code generated static route information structure.",null,null],[0,"import","gtfs_server::routes::api","`/import` related routes",null,null],[5,"url","gtfs_server::routes::api::import","",null,{"inputs":[{"generics":["routeshandler"],"name":"state"},{"name":"string"}],"output":{"generics":["successresult"],"name":"json"}}],[5,"agency","","",null,{"inputs":[{"generics":["routeshandler"],"name":"state"},{"name":"string"}],"output":{"generics":["successresult"],"name":"json"}}],[5,"stops","","",null,{"inputs":[{"generics":["routeshandler"],"name":"state"},{"name":"string"}],"output":{"generics":["successresult"],"name":"json"}}],[5,"times","","",null,{"inputs":[{"generics":["routeshandler"],"name":"state"},{"name":"string"}],"output":{"generics":["successresult"],"name":"json"}}],[5,"routes","","",null,{"inputs":[{"generics":["routeshandler"],"name":"state"},{"name":"string"}],"output":{"generics":["successresult"],"name":"json"}}],[5,"trips","","",null,{"inputs":[{"generics":["routeshandler"],"name":"state"},{"name":"string"}],"output":{"generics":["successresult"],"name":"json"}}],[5,"calendar","","",null,{"inputs":[{"generics":["routeshandler"],"name":"state"},{"name":"string"}],"output":{"generics":["successresult"],"name":"json"}}],[7,"static_rocket_route_info_for_url","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_agency","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_stops","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_times","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_routes","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_trips","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_calendar","","Rocket code generated static route information structure.",null,null],[0,"routes","gtfs_server::routes::api","`/routes` related routes",null,null],[5,"routes","gtfs_server::routes::api::routes","`/routes`   Returns a ResultArray <Route>",null,{"inputs":[{"generics":["routeshandler"],"name":"state"}],"output":{"generics":["resultarray"],"name":"json"}}],[5,"routes_by_query","","`/routes?query` Returns a ResultArray <Route>",null,{"inputs":[{"generics":["routeshandler"],"name":"state"},{"name":"routesearch"}],"output":{"generics":["resultarray"],"name":"json"}}],[5,"route_by_id","","`/routes/<route_uid>`   Gets the specified Route by its UID, parametrized as `<route_uid>`.   Returns a Result <Route>",null,{"inputs":[{"generics":["routeshandler"],"name":"state"},{"name":"string"}],"output":{"generics":["result"],"name":"json"}}],[5,"route_by_stop_uid","","`/routes/by-stop/<stop_uid>`   Gets the Routes that serve a particular Stop by its UID, parametrized as `<stop_uid>`.   Returns a Result <Route>",null,{"inputs":[{"generics":["routeshandler"],"name":"state"},{"name":"string"}],"output":{"generics":["resultarray"],"name":"json"}}],[7,"static_rocket_route_info_for_routes","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_routes_by_query","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_route_by_id","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_route_by_stop_uid","","Rocket code generated static route information structure.",null,null],[0,"stops","gtfs_server::routes::api","`/stops` related routes",null,null],[5,"stops","gtfs_server::routes::api::stops","`/stops`   Returns a ResultArray<Stop>",null,{"inputs":[{"generics":["routeshandler"],"name":"state"}],"output":{"generics":["resultarray"],"name":"json"}}],[5,"stops_by_id","","`/stops/<stop_id>`   Gets a single Stop from its `stop_id`.   Returns a Result<Stop>",null,{"inputs":[{"generics":["routeshandler"],"name":"state"},{"name":"string"}],"output":{"generics":["result"],"name":"json"}}],[5,"stops_by_trip","","`/stops/by-trip/<trip_id>`   get the Stops visited by a Trip uid.   Returns a ResultArray<Stop>",null,{"inputs":[{"generics":["routeshandler"],"name":"state"},{"name":"string"}],"output":{"generics":["resultarray"],"name":"json"}}],[5,"stops_near_default","","`/stops/near/<lat>/<lng>`   Gets an array of StopDistances, within 100.0 meters from , - nearest first.   Returns a ResultArray <StopDistance>",null,{"inputs":[{"generics":["routeshandler"],"name":"state"},{"name":"f32"},{"name":"f32"}],"output":{"generics":["resultarray"],"name":"json"}}],[5,"stops_near","","`/stops/near/<lat>/<lng>/<meters>`   Gets an array of StopDistances, within  meters from , nearest first, of Stops near the provided coordinate.   Returns a ResultArray <StopDistance>",null,{"inputs":[{"generics":["routeshandler"],"name":"state"},{"name":"f32"},{"name":"f32"},{"name":"f64"}],"output":{"generics":["resultarray"],"name":"json"}}],[7,"static_rocket_route_info_for_stops","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_stops_by_id","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_stops_by_trip","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_stops_near_default","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_stops_near","","Rocket code generated static route information structure.",null,null],[0,"times","gtfs_server::routes::api","`/times` related routes",null,null],[5,"times_query","gtfs_server::routes::api::times","",null,{"inputs":[{"generics":["routeshandler"],"name":"state"},{"name":"timesearch"}],"output":{"generics":["resultarray"],"name":"json"}}],[5,"times_by_trip","","`/times/by-trip/<trip_uid>`   Gets the Times associated to the specified Trip UID, parametrized as `<trip_id>`.   Returns a ResultArray <Time>",null,{"inputs":[{"generics":["routeshandler"],"name":"state"},{"name":"string"}],"output":{"generics":["resultarray"],"name":"json"}}],[5,"times_stop_query","","`/times/by-stop/<stop_id>?<time_search>`   Gets the Times associated to the specified Stop UID, parametrized as `<stop_id>`.   The results can be filtered with `<time_search>` parameters (check TimeSearch)   Returns a ResultArray <Time>",null,{"inputs":[{"generics":["routeshandler"],"name":"state"},{"name":"string"},{"name":"timesearch"}],"output":{"generics":["resultarray"],"name":"json"}}],[5,"times_stop","","`/times/by-stop/<stop_id>`   Gets the Times associated to the specified Stop UID, parametrized as `<stop_id>`.   Returns a ResultArray <Time>",null,{"inputs":[{"generics":["routeshandler"],"name":"state"},{"name":"string"}],"output":{"generics":["resultarray"],"name":"json"}}],[5,"get_times_by_trip","","",null,{"inputs":[{"name":"string"},{"name":"pool"}],"output":{"generics":["time"],"name":"vec"}}],[7,"static_rocket_route_info_for_times_query","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_times_by_trip","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_times_stop_query","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_times_stop","","Rocket code generated static route information structure.",null,null],[0,"trips","gtfs_server::routes::api","`/trips` related routes",null,null],[5,"trips","gtfs_server::routes::api::trips","`/trips/`, returns a list of Trips.   Returns a ResultArray <Trip>",null,{"inputs":[{"generics":["routeshandler"],"name":"state"}],"output":{"generics":["resultarray"],"name":"json"}}],[5,"trips_by_query","","`/trips/?query`, returns a list of Trips filtered with a TripSearch query. Returns a ResultArray <Trip>",null,{"inputs":[{"generics":["routeshandler"],"name":"state"},{"name":"tripsearch"}],"output":{"generics":["resultarray"],"name":"json"}}],[5,"trips_stopid","","`/trips/by-stop/<stop_id>`, returns the Trips associated to the specified Stop UID, parametrized as `<stop_id>`. Returns a ResultArray <Trip>",null,{"inputs":[{"generics":["routeshandler"],"name":"state"},{"name":"string"}],"output":{"generics":["resultarray"],"name":"json"}}],[5,"trip","","`/trips/<trip_id>`, returns the Trips associated to the specified Trip UID, parametrized as `<trip_id>`. Returns a Result <Trip>",null,{"inputs":[{"generics":["routeshandler"],"name":"state"},{"name":"string"}],"output":{"generics":["result"],"name":"json"}}],[5,"trip_by_route","","`/trips/by-route/<route_uid>`, returns the Trips associated to the specified Route UID, parametrized as `<route_uid>`. Returns a Result <Trip>",null,{"inputs":[{"generics":["routeshandler"],"name":"state"},{"name":"string"}],"output":{"generics":["resultarray"],"name":"json"}}],[7,"static_rocket_route_info_for_trips","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_trips_by_query","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_trips_stopid","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_trip","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_trip_by_route","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_main","gtfs_server::routes::api","Rocket code generated static route information structure.",null,null]],"paths":[[3,"Agency"],[3,"Error"],[3,"Meta"],[3,"Result"],[3,"ResultArray"],[4,"AscDesc"],[3,"RouteSearch"],[3,"TimeSearch"],[4,"TimeSort"],[3,"TripSearch"],[3,"StopDistance"],[3,"SuccessResult"],[3,"AgencyCSV"],[3,"CalendarCSV"],[3,"FeedCSV"],[3,"RouteCSV"],[3,"StopCSV"],[3,"StopTimeCSV"],[3,"TripCSV"],[4,"DropOff"],[4,"PickUp"],[3,"Route"],[3,"Stop"],[3,"StopTrip"],[3,"Time"],[3,"Trip"],[3,"RoutesHandler"]]};
initSearch(searchIndex);
