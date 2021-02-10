#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket_contrib::json::Json;

struct PostResponse {
    
}

#[get("/posts/<id>")]
fn blogview(id: u16) -> {
    let mut string = "?";
    content::Json(string)
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/", routes![blogview])
        .launch();
}