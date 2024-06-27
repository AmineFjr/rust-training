use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub subtitle: String,
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::posts)]
pub struct NewPost {
    pub title: String,
    pub body: String,
    pub published: bool,
    pub subtitle: String,
}
