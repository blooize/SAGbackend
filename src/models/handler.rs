    
use crate::connection::DbConn;
use diesel::result::Error;
use std::env;
use crate::models;
use crate::models::*;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

#[get("/")]
pub fn list_posts(connection: DbConn) -> Result<Json<Vec<Post>>, Status> {
    models::repository::all_posts(&connection)
        .map(|post| Json(post))
        .map_err(|error| error_status(error))
}

#[get("/<id>")]
pub fn get_post(id: i32, connection: DbConn) -> Result<Json<Post>, Status> {
    models::repository::get_post(id, &connection)
        .map(|post| Json(post))
        .map_err(|error| error_status(error))
}

#[post("/", format="application/json", data="<post>")]
pub fn post_post(post: Json<Post>, connection: DbConn) -> Result<status::Created<Json<Post>>, Status> {
    models::repository::insert_post(post.into_inner(), &connection)
        .map(|post| post_created(post))
        .map_err(|error| error_status(error))
}

#[put("/<id>", format="application/json", data="<post>")]
pub fn put_post(id: i32, post: Json<Post>, connection: DbConn) -> Result<Json<Post>, Status> {
    models::repository::update_post(id, post.into_inner(), &connection)
        .map(|post| Json(post))
        .map_err(|error| error_status(error))
}

#[delete("/<id>")]
pub fn delete_post(id: i32, connection: DbConn) -> Result<status::NoContent, Status> {
    match models::repository::get_post(id, &connection) {
        Ok(_) => models::repository::delete_post(id, &connection)
            .map(|_| status::NoContent)
            .map_err(|error| error_status(error)),
        Err(error) => Err(error_status(error))
    }
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}

pub fn post_created(post: Post) -> status::Created<Json<Post>> {
    let host = env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set");
    let port = env::var("ROCKET_PORT").expect("ROCKET_PORT must be set");
    status::Created(
        format!("{host}:{port}/post/{id}", host = host, port = port, id = post.id).to_string(),
        Some(Json(post))
    )
}

pub fn user_created(user: User) -> status::Created<Json<User>> {
    let host = env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set");
    let port = env::var("ROCKET_PORT").expect("ROCKET_PORT must be set");
    status::Created(
        format!("{host}:{port}/user/{id}", host = host, port = port, id = user.id).to_string(),
        Some(Json(user))
    )
}

#[get("/")]
pub fn list_users(connection: DbConn) ->  Result<Json<Vec<User>>, Status> {
    models::repository::all_users(&connection)
        .map(|user| Json(user))
        .map_err(|error| error_status(error))
}

#[get("/<id>")]
pub fn get_user(id: i32, connection: DbConn) -> Result<Json<User>, Status> {
    models::repository::get_user(id, &connection)
        .map(|user| Json(user))
        .map_err(|error| error_status(error))
}

#[post("/", format="application/json", data="<user>")]
pub fn post_user(user: Json<User>, connection: DbConn) -> Result<status::Created<Json<User>>, Status> {
    models::repository::insert_user(user.into_inner(), &connection)
        .map(|user| user_created(user))
        .map_err(|error| error_status(error))
}

#[put("/<id>", format="application/json", data="<user>")]
pub fn put_user(id: i32, user: Json<User>, connection: DbConn) -> Result<Json<User>, Status> {
    models::repository::update_user(id, user.into_inner(), &connection)
        .map(|user| Json(user))
        .map_err(|error| error_status(error))
}

#[delete("/<id>")]
pub fn delete_user(id: i32, connection: DbConn) -> Result<status::NoContent, Status> {
    match models::repository::get_user(id, &connection) {
        Ok(_) => models::repository::delete_user(id, &connection)
            .map(|_| status::NoContent)
            .map_err(|error| error_status(error)),
        Err(error) => Err(error_status(error))
    }
}
