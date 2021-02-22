use crate::schema::*;

use serde::Serialize;

#[derive(Debug, Queryable, Serialize)]
pub struct Post {
    pub id: i32, 
    pub title: String,
    pub body: String,
    pub published: bool
}

use super::schema::posts;

#[derive(Debug, Insertable, AsChangeset)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}