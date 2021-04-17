use rocket;

use crate::connection_handler;
use crate::db_handler;
use crate::request_handler;

pub fn create_routes() {
    rocket::ignite()
        .manage(connection_handler::init_pool())
        .mount(
            "/maps_with_id",
            routes![
                request_handler::get_all_maps,
                request_handler::create_map,
                request_handler::get_map,
                request_handler::update_map,
                request_handler::delete_map
            ],
        )
        .launch();
}
