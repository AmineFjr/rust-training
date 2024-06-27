use crate::models::reservation::{Reservation, NewReservation, SetReservation};
use crate::schema::reservations::dsl::{reservations, reservation_date, customer_id, table_id, party_size};
use crate::schema::tables::dsl::{seating_capacity, tables};
use crate::AppState;
use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;

pub async fn create_reservation(
    state: web::Data<AppState>,
    new_reservation: web::Json<NewReservation>,
) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

        let mut new_reservation = NewReservation {
            reservation_date: new_reservation.reservation_date.clone(),
            party_size: new_reservation.party_size.clone(),
            customer_id: new_reservation.customer_id.clone(),
            table_id: new_reservation.table_id.clone(),
        };

        let existing_reservation = reservations
            .filter(reservation_date.eq(new_reservation.reservation_date))
            .filter(customer_id.eq(new_reservation.customer_id))
            .filter(party_size.eq(new_reservation.party_size))
            .load::<Reservation>(&mut conn)
            .expect("Error loading reservations");
    
        if !existing_reservation.is_empty() {
            return HttpResponse::Conflict().body("Réservation déjà existante");
        }

        let search_available_table = tables
            .filter(seating_capacity.ge(new_reservation.party_size))
            .order(seating_capacity.asc())
            .load::<crate::models::table::Table>(&mut conn)
            .expect("Error loading tables");

        if search_available_table.is_empty() {
            return HttpResponse::NotFound().body("Aucune table disponible");
        }

        let table = search_available_table.first().unwrap();
        let reservation_table_id = table.id;
        
        new_reservation = NewReservation {
            reservation_date: new_reservation.reservation_date,
            party_size: new_reservation.party_size,
            customer_id: new_reservation.customer_id,
            table_id: reservation_table_id,
        };
        
    
        match diesel::insert_into(reservations)
            .values(&new_reservation)
            .get_result::<Reservation>(&mut conn)
        {
            Ok(reservation) => HttpResponse::Created().json(reservation),
            Err(err) => HttpResponse::InternalServerError().body(format!("Failed to insert reservation: {}", err)),
        }
}

pub async fn get_reservation(state: web::Data<AppState>, path: web::Path<(i32, i32)>) -> impl Responder {
    let (id_customer, id_table) = path.into_inner();
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

        match reservations
        .filter(customer_id.eq(id_customer).and(table_id.eq(id_table)))
        .first::<Reservation>(&mut conn) {
            Ok(reservation) => HttpResponse::Ok().json(reservation),
            Err(err) => HttpResponse::NotFound().body(format!("Réservation non trouvée : {}", err)),
    }
}

pub async fn get_reservations(state: web::Data<AppState>) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    match reservations.load::<Reservation>(&mut conn) {
        Ok(results) => HttpResponse::Ok().json(results),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to load reservations: {}", err))
        }
    }
}

pub async fn delete_reservation(state: web::Data<AppState>, path: web::Path<(i32, i32)>) -> impl Responder {
    let (id_customer, id_table) = path.into_inner();
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    match diesel::delete(reservations.filter(customer_id.eq(id_customer).and(table_id.eq(id_table)))).execute(&mut conn) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to delete reservation: {}", err))
        }
    }
}

pub async fn update_reservation(
    state: web::Data<AppState>,
    path: web::Path<(i32, i32)>,
    updated_reservation: web::Json<SetReservation>,
) -> impl Responder {
    let (id_customer, id_table) = path.into_inner();

    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    match diesel::update(reservations.filter(customer_id.eq(id_customer).and(table_id.eq(id_table))))
        .set((
            reservation_date.eq(&updated_reservation.reservation_date),
            party_size.eq(&updated_reservation.party_size),
        ))
        .execute(&mut conn)
    {
        Ok(_) => HttpResponse::Ok().json(updated_reservation.into_inner()),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to update reservation: {}", err))
        }
    }
}