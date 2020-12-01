#![feature(plugin, const_fn, decl_macro, proc_macro_hygiene)]
#![allow(proc_macro_derive_resolution_fallback, unused_attributes)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2_diesel;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

mod db;
mod models;
mod schema;
mod todolist_routes;
use todolist_routes::*;
fn rocket() -> rocket::Rocket {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("Database not foudn");

    let pool = db::init_pool(database_url);
    rocket::ignite()
        .manage(pool)
        .mount("/list", routes![lists, addlist, list_by_id, delete_list])
}

fn main() {
    rocket().launch();
}
