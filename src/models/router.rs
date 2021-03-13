use crate::models;
use rocket;
use crate::connection;
pub fn create_routes() {
    rocket::ignite()
        .manage(connection::init_pool())
        .mount("/posts", routes![
            models::handler::list_posts,
            models::handler::get_post,
            models::handler::post_post,
            models::handler::put_post,
            models::handler::delete_post,
        ])
        .mount("/users", routes![
            models::handler::list_users,
            models::handler::get_user,
            models::handler::post_user,
            models::handler::put_user,
            models::handler::delete_user,
        ])
        .launch();
}
