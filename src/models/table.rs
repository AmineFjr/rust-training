use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::tables)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Restaurant))]
pub struct Table {
    pub id: i32,
    pub seating_capacity: i32,
    pub restaurant_id: i32,
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::tables)]
#[diesel(belongs_to(Restaurant))]
pub struct NewTable {
    pub seating_capacity: i32,
    pub restaurant_id: i32,
}

#[derive(Deserialize, Serialize)]
pub struct SeatingCapacityTable {
    pub seating_capacity: i32,
}