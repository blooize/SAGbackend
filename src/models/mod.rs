use serde::{Deserialize, Serialize};
use super::schema::*;

pub mod handler;
pub mod router;
pub mod repository;

#[derive(Identifiable, Associations, Debug, Queryable, Serialize, Deserialize, AsChangeset)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String
}

#[derive(Associations, Identifiable, Debug, Queryable, Serialize, Deserialize, AsChangeset)]
#[table_name = "posts"]
#[belongs_to(User)]
pub struct Post {
    pub id: i32, 
    pub user_id: i32,
    pub title: String,
    pub body: String,
    pub summary: String,
    pub published: bool
}

