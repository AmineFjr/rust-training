use crate::models::restaurant::{Restaurant, NewRestaurant};
use crate::schema::restaurants::dsl::{restaurants};
use crate::AppState;
use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;

pub async fn create_restaurant(
    state: web::Data<AppState>,
    new_restaurant: web::Json<NewRestaurant>,
) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    match diesel::insert_into(restaurants)
        .values(&new_restaurant.into_inner())
        .get_result::<Restaurant>(&mut conn)
    {
        Ok(restaurant) => HttpResponse::Created().json(restaurant),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to insert restaurant: {}", err))
        }
    }
}

pub async fn get_restaurant(state: web::Data<AppState>, restaurant_id: web::Path<i32>) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    match restaurants.find(*restaurant_id).first::<Restaurant>(&mut conn) {
        Ok(restaurant) => HttpResponse::Ok().json(restaurant),
        Err(err) => HttpResponse::NotFound().body(format!("Restaurant not found: {}", err)),
    }
}

pub async fn get_restaurants(state: web::Data<AppState>) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    match restaurants.load::<Restaurant>(&mut conn) {
        Ok(results) => HttpResponse::Ok().json(results),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to load restaurants: {}", err))
        }
    }
}

pub async fn delete_restaurant(state: web::Data<AppState>, restaurant_id: web::Path<i32>) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    match diesel::delete(restaurants.find(*restaurant_id)).execute(&mut conn) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to delete restaurant: {}", err))
        }
    }
}