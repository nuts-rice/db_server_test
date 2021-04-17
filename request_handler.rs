use std::env;

use diesel::result::Error;
use rocket::http::RawStr;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

use crate::db_handler;

use crate::model::Map;
use crate::model::NewMap;

use crate::connection_handler::DbConn;

//TODO: fix error staus and map created in closures

#[get("/index")]
pub fn get_all_maps(connection: DbConn) -> Result<Json<Vec<Map>>, Status> {
    db_handler::show_maps(&connection)
        .map(|map| Json(map))
        .map_err(|error| error_status(error))
}
#[post("/", format = "application/json", data = "<new_map>")]
pub fn create_map(
    new_map: Json<NewMap>,
    connection: DbConn,
) -> Result<status::Created<Json<Map>>, Status> {
    println!("new map created with key {}", &new_map.key);
    db_handler::create_map(new_map.into_inner(), &connection)
        .map(|map| map_created(map))
        .map_err(|error| error_status(error))
}

#[get("/get?<key>")]
//The key parameter here are request guards, a parameter injected by rocket
//if they fail to be created, ErrorCatcher is invoked. It breaks gracefully if it doesn't fit what is expected. This is Rust's type safety at work!
pub fn get_map(key: String, connection: DbConn) -> Result<Json<Map>, Status> {
    db_handler::get_map(key, &connection)
        .map(|map| Json(map))
        .map_err(|error| error_status(error))
}

#[put("/set?<key>", data = "<map>")]
pub fn update_map(key: String, map: Json<Map>, connection: DbConn) -> Result<Json<Map>, Status> {
    db_handler::update_map(key, map.into_inner(), &connection)
        .map(|map| Json(map))
        .map_err(|error| error_status(error))
}

#[delete("/delete?<key>")]
pub fn delete_map(key: String, connection: DbConn) -> Result<status::NoContent, Status> {
    db_handler::delete_map(key, &connection)
        .map(|_| status::NoContent)
        .map_err(|error| error_status(error))
}

//Matches for 201 Created status
fn map_created(map: Map) -> status::Created<Json<Map>> {
    status::Created(
        format!(
            "{host}:{port}/map/",
            host = host(),
            port = port(),
        )
        .to_string(),
        Some(Json(map)),
    )
}

//looks for env variable for host, returns as String
fn host() -> String {
    env::var("ROCKET_ADDRESS").expect("ROCKET ADDRESS must be set")
}

fn port() -> String {
    env::var("ROCKET_PORT").expect("ROCKET PORT must be set")
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError,
    }
}
