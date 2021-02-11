#[derive(Queryable, Identifiable)]
#[belongs_to(User)]
pub struct Post {
    pub id: i32, 
    pub title: String,
    pub body: String,
    pub author_id: User,
    pub published: bool
}

#[derive(Queryable, Associations, Identifiable)]
pub struct User {
    pub id: i32, 
    pub name: String,
    pub email: String,
    pub password: String
}