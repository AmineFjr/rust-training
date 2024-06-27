use crate::models::table::{Table, NewTable};
use crate::schema::tables::dsl::{tables};
use crate::AppState;
use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;

pub async fn create_table(
    state: web::Data<AppState>,
    new_table: web::Json<NewTable>,
) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    match diesel::insert_into(tables)
        .values(&new_table.into_inner())
        .get_result::<Table>(&mut conn)
    {
        Ok(table) => HttpResponse::Created().json(table),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to insert table: {}", err))
        }
    }
}

pub async fn get_table(state: web::Data<AppState>, table_id: web::Path<i32>) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    match tables.find(*table_id).first::<Table>(&mut conn) {
        Ok(table) => HttpResponse::Ok().json(table),
        Err(err) => HttpResponse::NotFound().body(format!("Table not found: {}", err)),
    }
}

pub async fn get_tables(state: web::Data<AppState>) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    match tables.load::<Table>(&mut conn) {
        Ok(results) => HttpResponse::Ok().json(results),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to load tables: {}", err))
        }
    }
}

pub async fn delete_table(state: web::Data<AppState>, table_id: web::Path<i32>) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    match diesel::delete(tables.find(*table_id)).execute(&mut conn) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to delete table: {}", err))
        }
    }
}