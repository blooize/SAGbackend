use crate::schema::*;
use serde::{Deserialize, Serialize};
use super::schema::posts;

pub mod handler;
pub mod router;
pub mod repository;

#[derive(Debug, Queryable, Serialize, Deserialize, AsChangeset)]
pub struct Post {
    pub id: i32, 
    pub title: String,
    pub body: String,
    pub summary: String,
    pub published: bool
}

