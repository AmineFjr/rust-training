use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Debug)]
#[diesel(table_name = crate::schema::comments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Post))]
pub struct Comment {
    pub id: i32,
    pub content: String,
    pub user_id: i32,
    pub post_id: i32,
}

#[derive(Insertable, Deserialize, Serialize, Debug)]
#[diesel(table_name = crate::schema::comments)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Post))]
pub struct NewComment {
    pub content: String,
    pub user_id: i32,
    pub post_id: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Content {
    pub content: String,
}
