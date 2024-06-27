use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::reservations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Table))]
#[diesel(belongs_to(Customer))]
pub struct Reservation {
    pub id: i32,
    pub reservation_date: chrono::NaiveDate,
    pub party_size: i32,
    pub table_id: i32,
    pub customer_id: i32,
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::reservations)]
pub struct NewReservation {
    pub reservation_date: chrono::NaiveDate,
    pub party_size: i32,
    pub customer_id: i32,
    pub table_id: i32,
}

#[derive(Deserialize, Serialize)]
pub struct SetReservation {
    pub reservation_date: chrono::NaiveDate,
    pub party_size: i32,
}