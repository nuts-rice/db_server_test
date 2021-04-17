#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate diesel;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde_derive;

use log::info;
use std::env;

mod connection_handler;
mod db_handler;
mod model;
mod request_handler;
mod routes;
mod schema;

//TODO:
//use rust-gdb/cargo expand to finetune and figure it out
//review the documentation
//Demonstrate in-memory caching

//Checking for Ok on variables defined in .env
fn init_env() {
    dotenv::dotenv().ok();
}

fn main() {
    init_env();
    routes::create_routes();
}
