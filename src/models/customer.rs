use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Debug)]
#[diesel(table_name = crate::schema::customers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Customer {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Insertable, Deserialize, Serialize, Debug)]
#[diesel(table_name = crate::schema::customers)]
pub struct NewCustomer {
    pub name: String,
    pub email: String,
}
