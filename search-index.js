var searchIndex = {};
searchIndex["gtfs_server"] = {"doc":"This is the documentation for `gtfs-server`","items":[[0,"models","gtfs_server","This namespace represents the models used in the API, this part of the documentation should be used as a reference for your classes in your application's wrapper. Please note that some `models` contain a `feed_id` and a `model_id`. These fields are not  serialized.",null,null],[0,"api","gtfs_server::models","API related models",null,null],[0,"error","gtfs_server::models::api","Error related structs and implementations",null,null],[3,"Error","gtfs_server::models::api::error","",null,null],[12,"code","","",0,null],[12,"message","","",0,null],[11,"fmt","","",0,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[0,"meta","gtfs_server::models::api","Meta related structs and implementations",null,null],[3,"Meta","gtfs_server::models::api::meta","",null,null],[12,"success","","",1,null],[12,"error","","",1,null],[11,"fmt","","",1,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[0,"result","gtfs_server::models::api","Result related structs and implementations",null,null],[3,"Result","gtfs_server::models::api::result","",null,null],[12,"result","","",2,null],[12,"meta","","",2,null],[11,"fmt","","",2,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[0,"resultarray","gtfs_server::models::api","ResultArray related structs and implementations",null,null],[3,"ResultArray","gtfs_server::models::api::resultarray","",null,null],[12,"result","","",3,null],[12,"meta","","",3,null],[11,"fmt","","",3,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[0,"search","gtfs_server::models::api","Search Related Models (Query Parameters)",null,null],[0,"ascdesc","gtfs_server::models::api::search","",null,null],[4,"AscDesc","gtfs_server::models::api::search::ascdesc","",null,null],[13,"ASC","","",4,null],[13,"DESC","","",4,null],[11,"as_str","","",4,{"inputs":[{"name":"self"}],"output":{"name":"str"}}],[0,"time","gtfs_server::models::api::search","TimeSearch related structs and implementations",null,null],[3,"TimeSearch","gtfs_server::models::api::search::time","",null,null],[12,"date","","",5,null],[12,"service_uid","","",5,null],[12,"monday","","",5,null],[12,"tuesday","","",5,null],[12,"wednesday","","",5,null],[12,"thursday","","",5,null],[12,"friday","","",5,null],[12,"saturday","","",5,null],[12,"sunday","","",5,null],[12,"at_a","","",5,null],[12,"at_b","","",5,null],[12,"dt_a","","",5,null],[12,"dt_b","","",5,null],[12,"trip_id","","",5,null],[12,"pickup_type","","",5,null],[12,"drop_off_type","","",5,null],[12,"stop_sequence","","",5,null],[12,"sort_by","","",5,null],[12,"sort_order","","",5,null],[4,"TimeSort","","",null,null],[13,"arrival_time","","",6,null],[13,"departure_time","","",6,null],[13,"stop_sequence","","",6,null],[11,"from_form","","",5,{"inputs":[{"name":"formitems"},{"name":"bool"}],"output":{"generics":["timesearch","error"],"name":"result"}}],[11,"as_str","","",6,{"inputs":[{"name":"self"}],"output":{"name":"str"}}],[0,"stopdistance","gtfs_server::models::api","StopDistance related structs and implementations",null,null],[3,"StopDistance","gtfs_server::models::api::stopdistance","",null,null],[12,"stop","","",7,null],[12,"distance","","",7,null],[11,"fmt","","",7,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[0,"successresult","gtfs_server::models::api","SuccessResult related structs and implementations",null,null],[3,"SuccessResult","gtfs_server::models::api::successresult","",null,null],[12,"success","","",8,null],[11,"fmt","","",8,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[0,"agency","gtfs_server::models","Agency related structs and implementations",null,null],[3,"Agency","gtfs_server::models::agency","",null,null],[12,"uid","","",9,null],[12,"id","","",9,null],[12,"name","","",9,null],[12,"url","","",9,null],[12,"timezone","","",9,null],[12,"lang","","",9,null],[12,"phone","","",9,null],[12,"feed_id","","",9,null],[11,"fmt","","",9,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[0,"csv","gtfs_server::models","This module represents the entities as found in the CSV files.   These structs are only used for CSV-parsing. Therefore they are conform to the GTFS reference.  ",null,null],[0,"agency","gtfs_server::models::csv","",null,null],[3,"AgencyCSV","gtfs_server::models::csv::agency","",null,null],[12,"agency_id","","",10,null],[12,"agency_name","","",10,null],[12,"agency_url","","",10,null],[12,"agency_timezone","","",10,null],[12,"agency_lang","","",10,null],[12,"agency_phone","","",10,null],[11,"fmt","","",10,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[0,"feed","gtfs_server::models::csv","",null,null],[3,"FeedCSV","gtfs_server::models::csv::feed","",null,null],[12,"feed_publisher_name","","",11,null],[12,"feed_publisher_url","","",11,null],[12,"feed_lang","","",11,null],[12,"feed_start_date","","",11,null],[12,"feed_end_date","","",11,null],[12,"feed_version","","",11,null],[11,"fmt","","",11,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[0,"route","gtfs_server::models::csv","",null,null],[3,"RouteCSV","gtfs_server::models::csv::route","",null,null],[12,"route_id","","",12,null],[12,"agency_id","","",12,null],[12,"route_short_name","","",12,null],[12,"route_long_name","","",12,null],[12,"route_desc","","",12,null],[12,"route_type","","",12,null],[11,"fmt","","",12,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[0,"stop","gtfs_server::models::csv","",null,null],[3,"StopCSV","gtfs_server::models::csv::stop","",null,null],[12,"stop_id","","",13,null],[12,"stop_name","","",13,null],[12,"stop_lat","","",13,null],[12,"stop_lon","","",13,null],[12,"location_type","","",13,null],[12,"parent_station","","",13,null],[11,"fmt","","",13,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[0,"stoptime","gtfs_server::models::csv","",null,null],[3,"StopTimeCSV","gtfs_server::models::csv::stoptime","",null,null],[12,"trip_id","","",14,null],[12,"arrival_time","","",14,null],[12,"departure_time","","",14,null],[12,"stop_id","","",14,null],[12,"stop_sequence","","",14,null],[12,"pickup_type","","",14,null],[12,"drop_off_type","","",14,null],[11,"fmt","","",14,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[0,"trip","gtfs_server::models::csv","",null,null],[3,"TripCSV","gtfs_server::models::csv::trip","",null,null],[12,"route_id","","",15,null],[12,"service_id","","",15,null],[12,"trip_id","","",15,null],[12,"trip_headsign","","",15,null],[12,"trip_short_name","","",15,null],[12,"direction_id","","",15,null],[11,"fmt","","",15,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[0,"calendar","gtfs_server::models::csv","",null,null],[3,"CalendarCSV","gtfs_server::models::csv::calendar","",null,null],[12,"service_id","","",16,null],[12,"monday","","",16,null],[12,"tuesday","","",16,null],[12,"wednesday","","",16,null],[12,"thursday","","",16,null],[12,"friday","","",16,null],[12,"saturday","","",16,null],[12,"sunday","","",16,null],[12,"start_date","","",16,null],[12,"end_date","","",16,null],[11,"fmt","","",16,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[0,"dropoff","gtfs_server::models","DropOff related enums and implementations",null,null],[4,"DropOff","gtfs_server::models::dropoff","",null,null],[13,"RegularlyScheduled","","",17,null],[13,"NotAvailable","","",17,null],[13,"MustArrangeWithAgency","","",17,null],[13,"MustCoordinateWithDriver","","",17,null],[11,"fmt","","",17,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[0,"pickup","gtfs_server::models","PickUp related enums and implementations",null,null],[4,"PickUp","gtfs_server::models::pickup","",null,null],[13,"RegularlyScheduled","","",18,null],[13,"NoPickupAvailable","","",18,null],[13,"MustArrangeWithAgency","","",18,null],[13,"MustCoordinateWithDriver","","",18,null],[11,"fmt","","",18,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[0,"route","gtfs_server::models","Route related structs and implementations",null,null],[3,"Route","gtfs_server::models::route","",null,null],[12,"uid","","",19,null],[12,"id","","",19,null],[12,"agency_id","","",19,null],[12,"short_name","","",19,null],[12,"long_name","","",19,null],[12,"description","","",19,null],[12,"route_type","","",19,null],[12,"feed_id","","",19,null],[11,"fmt","","",19,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[0,"stop","gtfs_server::models","Stop related structs and implementations",null,null],[3,"Stop","gtfs_server::models::stop","",null,null],[12,"uid","","",20,null],[12,"name","","",20,null],[12,"lat","","",20,null],[12,"lng","","",20,null],[12,"location_type","","",20,null],[12,"parent_station","","",20,null],[3,"StopTrip","","",null,null],[12,"stop","","",21,null],[12,"arrival_time","","",21,null],[12,"departure_time","","",21,null],[12,"stop_sequence","","",21,null],[12,"drop_off","","",21,null],[12,"pickup","","",21,null],[11,"fmt","","",20,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"fmt","","",21,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"new","","",20,{"inputs":[{"name":"string"},{"name":"string"},{"name":"f64"},{"name":"f64"},{"name":"i32"},{"generics":["string"],"name":"option"}],"output":{"name":"stop"}}],[11,"set_id","","",20,{"inputs":[{"name":"self"},{"name":"string"}],"output":null}],[11,"set_feed_id","","",20,{"inputs":[{"name":"self"},{"name":"string"}],"output":null}],[0,"time","gtfs_server::models","Time related structs and implementations",null,null],[3,"Time","gtfs_server::models::time","",null,null],[12,"trip_id","","",22,null],[12,"arrival_time","","",22,null],[12,"departure_time","","",22,null],[12,"stop_id","","",22,null],[12,"stop_sequence","","",22,null],[12,"pickup_type","","",22,null],[12,"drop_off_type","","",22,null],[12,"service_days","","",22,null],[12,"service_uid","","",22,null],[12,"start_date","","",22,null],[12,"end_date","","",22,null],[12,"feed_id","","",22,null],[11,"fmt","","",22,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[0,"trip","gtfs_server::models","Trip related structs and implementations",null,null],[3,"Trip","gtfs_server::models::trip","",null,null],[12,"uid","","",23,null],[12,"route_id","","",23,null],[12,"service_id","","",23,null],[12,"headsign","","",23,null],[12,"short_name","","",23,null],[12,"direction_id","","",23,null],[12,"stop_sequence","","",23,null],[11,"fmt","","",23,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"new","","",23,{"inputs":[{"name":"string"},{"name":"string"},{"name":"string"},{"name":"string"},{"name":"string"},{"name":"i32"}],"output":{"name":"trip"}}],[11,"set_id","","",23,{"inputs":[{"name":"self"},{"name":"string"}],"output":null}],[11,"set_feed_id","","",23,{"inputs":[{"name":"self"},{"name":"string"}],"output":null}],[0,"routes","gtfs_server","This model represents all the routes managed by Rocket.   Some routes may not be active: you may want to check main.rs for a list of enabled routes.",null,null],[3,"RoutesHandler","gtfs_server::routes","",null,null],[12,"pool","","",24,null],[0,"api","","",null,null],[5,"main","gtfs_server::routes::api","",null,{"inputs":[],"output":{"name":"string"}}],[0,"agency","","`/agency` related routes",null,null],[5,"agency_by_id","gtfs_server::routes::api::agency","`/agency/<agency_uid>`   Get the the specified Agency by its specified UID.   Returns a Result<Agency>",null,{"inputs":[{"generics":["routeshandler"],"name":"state"},{"name":"string"}],"output":{"generics":["result"],"name":"json"}}],[5,"get_agency_id","","Returns the UID of the `agency_id` and `feed_id` provided.",null,{"inputs":[{"generics":["string"],"name":"option"},{"name":"string"},{"name":"state"}],"output":{"generics":["string"],"name":"option"}}],[7,"static_rocket_route_info_for_agency_by_id","","Rocket code generated static route information structure.",null,null],[0,"import","gtfs_server::routes::api","`/import` related routes",null,null],[5,"agency","gtfs_server::routes::api::import","",null,{"inputs":[{"generics":["routeshandler"],"name":"state"},{"name":"string"}],"output":{"generics":["successresult"],"name":"json"}}],[5,"stops","","",null,{"inputs":[{"generics":["routeshandler"],"name":"state"},{"name":"string"}],"output":{"generics":["successresult"],"name":"json"}}],[5,"routes","","",null,{"inputs":[{"generics":["routeshandler"],"name":"state"},{"name":"string"}],"output":{"generics":["successresult"],"name":"json"}}],[5,"trips","","",null,{"inputs":[{"generics":["routeshandler"],"name":"state"},{"name":"string"}],"output":{"generics":["successresult"],"name":"json"}}],[5,"calendar","","",null,{"inputs":[{"generics":["routeshandler"],"name":"state"},{"name":"string"}],"output":{"generics":["successresult"],"name":"json"}}],[7,"static_rocket_route_info_for_agency","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_stops","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_routes","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_trips","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_calendar","","Rocket code generated static route information structure.",null,null],[0,"routes","gtfs_server::routes::api","`/routes` related routes",null,null],[5,"routes","gtfs_server::routes::api::routes","`/routes`   Returns a ResultArray <Route>",null,{"inputs":[{"generics":["routeshandler"],"name":"state"}],"output":{"generics":["resultarray"],"name":"json"}}],[5,"route_by_id","","`/routes/<route_uid>`   Gets the specified Route by its UID, parametrized as `<route_uid>`.   Returns a Result <Route>",null,{"inputs":[{"generics":["routeshandler"],"name":"state"},{"name":"string"}],"output":{"generics":["result"],"name":"json"}}],[7,"static_rocket_route_info_for_routes","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_route_by_id","","Rocket code generated static route information structure.",null,null],[0,"stops","gtfs_server::routes::api","`/stops` related routes",null,null],[5,"stops","gtfs_server::routes::api::stops","`/stops`   Returns a ResultArray<Stop>",null,{"inputs":[{"generics":["routeshandler"],"name":"state"}],"output":{"generics":["resultarray"],"name":"json"}}],[5,"stops_by_id","","`/stops/<stop_id>`   Gets a single Stop from its `stop_id`.   Returns a Result<Stop>",null,{"inputs":[{"generics":["routeshandler"],"name":"state"},{"name":"string"}],"output":{"generics":["result"],"name":"json"}}],[5,"stops_by_trip","","`/stops/by-trip/<trip_id>`   get the Stops visited by a Trip uid.   Returns a ResultArray<Stop>",null,{"inputs":[{"generics":["routeshandler"],"name":"state"},{"name":"string"}],"output":{"generics":["resultarray"],"name":"json"}}],[5,"stops_near_default","","`/stops/near/<lat>/<lng>`   Gets an array of StopDistances, within 100.0 meters from , - nearest first.   Returns a ResultArray <StopDistance>",null,{"inputs":[{"generics":["routeshandler"],"name":"state"},{"name":"f32"},{"name":"f32"}],"output":{"generics":["resultarray"],"name":"json"}}],[5,"stops_near","","`/stops/near/<lat>/<lng>/<meters>`   Gets an array of StopDistances, within  meters from , nearest first, of Stops near the provided coordinate.   Returns a ResultArray <StopDistance>",null,{"inputs":[{"generics":["routeshandler"],"name":"state"},{"name":"f32"},{"name":"f32"},{"name":"f64"}],"output":{"generics":["resultarray"],"name":"json"}}],[7,"static_rocket_route_info_for_stops","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_stops_by_id","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_stops_by_trip","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_stops_near_default","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_stops_near","","Rocket code generated static route information structure.",null,null],[0,"trips","gtfs_server::routes::api","`/trips` related routes",null,null],[5,"trips","gtfs_server::routes::api::trips","`/trips/`, returns a list of Trips.   Returns a ResultArray <Trip>",null,{"inputs":[{"generics":["routeshandler"],"name":"state"}],"output":{"generics":["resultarray"],"name":"json"}}],[5,"trips_stopid","","`/trips/by-stop/<stop_id>`, returns the Trips associated to the specified Stop UID, parametrized as `<stop_id>`. Returns a ResultArray <Trip>",null,{"inputs":[{"generics":["routeshandler"],"name":"state"},{"name":"string"}],"output":{"generics":["resultarray"],"name":"json"}}],[5,"trip","","`/trips/<trip_id>`, returns the Trips associated to the specified Trip UID, parametrized as `<trip_id>`. Returns a Result <Trip>",null,{"inputs":[{"generics":["routeshandler"],"name":"state"},{"name":"string"}],"output":{"generics":["result"],"name":"json"}}],[7,"static_rocket_route_info_for_trips","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_trips_stopid","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_trip","","Rocket code generated static route information structure.",null,null],[0,"times","gtfs_server::routes::api","`/times` related routes",null,null],[5,"times_trip","gtfs_server::routes::api::times","`/times/by-trip/<trip_id>`   Gets the Times associated to the specified Trip UID, parametrized as `<trip_id>`.   Returns a ResultArray <Time>",null,{"inputs":[{"generics":["routeshandler"],"name":"state"},{"name":"string"}],"output":{"generics":["resultarray"],"name":"json"}}],[5,"times_stop_query","","`/times/by-stop/<stop_id>`   Gets the Times associated to the specified Stop UID, parametrized as `<stop_id>`.   The results can be filtered with `<time_search>` parameters (check TimeSearch)   Returns a ResultArray <Time>",null,{"inputs":[{"generics":["routeshandler"],"name":"state"},{"name":"string"},{"name":"timesearch"}],"output":{"generics":["resultarray"],"name":"json"}}],[5,"times_stop","","`/times/by-stop/<stop_id>?<time_search>`   Gets the Times associated to the specified Stop UID, parametrized as `<stop_id>`.   Returns a ResultArray <Time>",null,{"inputs":[{"generics":["routeshandler"],"name":"state"},{"name":"string"}],"output":{"generics":["resultarray"],"name":"json"}}],[5,"get_times_by_trip","","",null,{"inputs":[{"name":"string"},{"name":"pool"}],"output":{"generics":["time"],"name":"vec"}}],[7,"static_rocket_route_info_for_times_trip","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_times_stop_query","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_times_stop","","Rocket code generated static route information structure.",null,null],[7,"static_rocket_route_info_for_main","gtfs_server::routes::api","Rocket code generated static route information structure.",null,null]],"paths":[[3,"Error"],[3,"Meta"],[3,"Result"],[3,"ResultArray"],[4,"AscDesc"],[3,"TimeSearch"],[4,"TimeSort"],[3,"StopDistance"],[3,"SuccessResult"],[3,"Agency"],[3,"AgencyCSV"],[3,"FeedCSV"],[3,"RouteCSV"],[3,"StopCSV"],[3,"StopTimeCSV"],[3,"TripCSV"],[3,"CalendarCSV"],[4,"DropOff"],[4,"PickUp"],[3,"Route"],[3,"Stop"],[3,"StopTrip"],[3,"Time"],[3,"Trip"],[3,"RoutesHandler"]]};
initSearch(searchIndex);
