use crate::posts;
use rocket;
use crate::connection;
pub fn create_routes() {
    rocket::ignite()
        .manage(connection::init_pool())
        .mount("/posts", routes![
            posts::handler::list,
            posts::handler::get,
            posts::handler::post,
            posts::handler::put,
            posts::handler::delete,
        ])
        .launch();
}
