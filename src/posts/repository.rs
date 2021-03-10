#![allow(proc_macro_derive_resolution_fallback)]
use diesel;
use diesel::prelude::*;
use crate::schema::posts;
use crate::posts::Post;

pub fn all(connections: &PgConnection) -> QueryResult<Vec<Post>>{
    posts::table.load::<Post>(&*connections)
}

pub fn get(id: i32, connection: &PgConnection) -> QueryResult<Post>{
    posts::table.find(id).get_result::<Post>(connection)
}

pub fn insert(post: Post, connection: &PgConnection) -> QueryResult<Post>{
    diesel::insert_into(posts::table)
        .values(&InsertablePost::from_post(post))
        .get_result(connection)
}

pub fn update(id: i32, post: Post, connection: &PgConnection) -> QueryResult<Post>{
    diesel::update(posts::table.find(id))
        .set(&post)
        .get_result(connection)
}

pub fn delete(id: i32, connection: &PgConnection) -> QueryResult<usize>{
    diesel::delete(posts::table.find(id))
        .execute(connection)
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct InsertablePost {
    pub title: String,
    pub body: String,
    pub summary: String
}

impl InsertablePost{
    fn from_post(post: Post) -> InsertablePost{
        InsertablePost{
            title: post.title,
            body: post.body,
            summary: post.summary 
        }
    }
}
