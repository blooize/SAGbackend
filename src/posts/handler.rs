    
use crate::connection::DbConn;
use diesel::result::Error;
use std::env;
use crate::posts;
use crate::posts::Post;
use rocket::http::Status;
use rocket::response::{status, Failure};
use rocket_contrib::json::Json;

#[get("/")]
pub fn list(connection: DbConn) -> Result<Json<Vec<Post>>, Failure> {
    posts::repository::all(&connection)
        .map(|post| Json(post))
        .map_err(|error| error_status(error))
}

fn error_status(error: Error) -> Failure {
    Failure(match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    });
}

#[get("/<id>")]
pub fn get(id: i32, connection: DbConn) -> Result<Json<Post>, Failure> {
    posts::repository::get(id, &connection)
        .map(|post| Json(post))
        .map_err(|error| error_status(error))
}

#[post("/", format="application/json", data="<post>")]
pub fn post(post: Json<Post>, connection: DbConn) -> Result<status::Created<Json<Post>>, Failure> {
    posts::repository::insert(post.into_inner(), &connection)
        .map(|post| post_created(post))
}

pub fn post_created(post: Post) -> status::Created<Json<Post>> {
    let host = env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set");
    let port = env::var("ROCKET_PORT").expect("ROCKET_PORT must be set");
    status::Created(
        format!("{host}:{port}/post/{id}", host = host, port = port, id = post.id).to_string(),
        Some(Json(post))
    )
}


fn host() -> String {
    env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set")
}

fn port() -> String {
    env::var("ROCKET_PORT").expect("ROCKET_PORT must be set")
}

#[put("/<id>", format="application/json", data="<post>")]
pub fn put(id: i32, post: Json<Post>, connection: DbConn) -> Result<Json<Post>, Failure> {
    posts::repository::update(id, post.into_inner(), &connection)
        .map(|post| Json(post))
        .map_err(|error| error_status(error))
}

#[delete("/<id>")]
pub fn delete(id: i32, connection: DbConn) -> Result<Json<Post>, Failure>{
    match posts::repository::get(id, &connection) {
        Ok(_) => posts::repository::delete(id, &connection)
            .map(|_| status::NoContent)
            .map_err(|error| error_status(error)),
        Err(error) => Err(error_status(error))
    }
}