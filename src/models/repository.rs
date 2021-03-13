#![allow(proc_macro_derive_resolution_fallback)]
use std::usize;

use diesel;
use diesel::prelude::*;
use crate::schema::{users, posts};
use crate::models::*;

pub fn all_users(connection: &PgConnection) -> QueryResult<Vec<User>> {
    users::table.load::<User>(&*connection)
}

pub fn all_posts(connection: &PgConnection) -> QueryResult<Vec<Post>>{
    posts::table.load::<Post>(&*connection)
}

pub fn get_user(id: i32, connection: &PgConnection) -> QueryResult<User> {
    users::table.find(id).get_result::<User>(connection)
}

pub fn get_post(id: i32, connection: &PgConnection) -> QueryResult<Post>{
    posts::table.find(id).get_result::<Post>(connection)
}

pub fn insert_user(user: User, connection: &PgConnection) -> QueryResult<User> {
    diesel::insert_into(users::table)
        .values(&InsertableUser::from_user(user))
        .get_result(connection)
}

pub fn insert_post(post: Post, connection: &PgConnection) -> QueryResult<Post>{
    diesel::insert_into(posts::table)
        .values(&InsertablePost::from_post(post))
        .get_result(connection)
}

pub fn update_user(id: i32, user: User, connection: &PgConnection) -> QueryResult<User> {
    diesel::update(users::table.find(id))
        .set(&user)
        .get_result(connection)
}

pub fn update_post(id: i32, post: Post, connection: &PgConnection) -> QueryResult<Post>{
    diesel::update(posts::table.find(id))
        .set(&post)
        .get_result(connection)
}

pub fn delete_user(id: i32, connection: &PgConnection) -> QueryResult<usize>{
    diesel::delete(users::table.find(id))
        .execute(connection)
}

pub fn delete_post(id: i32, connection: &PgConnection) -> QueryResult<usize>{
    diesel::delete(posts::table.find(id))
        .execute(connection)
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct InsertableUser {
    pub name: String, 
    pub email: String, 
    pub password: String
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct InsertablePost {
    pub title: String,
    pub body: String,
    pub summary: String
}

impl InsertableUser{
    fn from_user(user: User) -> InsertableUser{
        InsertableUser{
            name: user.name,
            email: user.email,
            password: user.password
        }
    }
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
