
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;
extern crate dotenv;


use diesel::pg::PgConnection;
use diesel::Connection;
use dotenv::dotenv;
use std::env;
use rocket_contrib::templates::Template;

pub mod posts;

pub mod models;

pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[get("/posts/<id>")]
fn blogview(id: i32) -> &'static str {
    "What {}", id
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![
            index,
            blogview,
            posts::list,
            posts::new,
            posts::insert,
            posts::update,
            posts::delete,
            posts::process_update
            ])
        .attach(Template::fairing())
        .launch();
}