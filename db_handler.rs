#![allow(proc_macro_derive_resolution_fallback)]
use diesel;
use diesel::prelude::*;

use crate::model::Map;
use crate::model::NewMap;
use rocket::{get, routes, uri};

use crate::schema::id_maps;
use crate::schema::id_maps::dsl::*;
//Handler in Rocket is a function that takes parameter bindings and handles the mapped request
//Defining db CRUD operations here

pub fn create_map(new_map: NewMap, conn: &PgConnection) -> QueryResult<Map> {
    diesel::insert_into(id_maps::table)
        .values(&new_map)
        .get_result(conn)
}

pub fn show_maps(conn: &PgConnection) -> QueryResult<Vec<Map>> {
    //maps.filter(published.eq(true))
    id_maps.limit(10).load::<Map>(&*conn)
}
//Need to define map_key with trait QueryId
pub fn get_map(map_key: String, conn: &PgConnection) -> QueryResult<Map> {
    id_maps::table.find(map_key).get_result::<Map>(conn)
}

pub fn update_map(map_key: String, map: Map, conn: &PgConnection) -> QueryResult<Map> {
    diesel::update(id_maps::table.find(map_key))
        .set(&map)
        .get_result(conn)
}

pub fn delete_map(map_key: String, conn: &PgConnection) -> QueryResult<Map> {
    diesel::delete(id_maps::table.find(map_key)).get_result(conn)
}
