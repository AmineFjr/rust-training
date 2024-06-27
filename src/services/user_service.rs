use crate::models::user::{NewUser, User};
use crate::schema::users::dsl::{full_name, id, users};
use crate::AppState;
use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;

pub async fn create_user(
    state: web::Data<AppState>,
    new_user: web::Json<NewUser>,
) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    let new_user = NewUser {
        full_name: new_user.full_name.clone(),
    };

    match diesel::insert_into(users)
        .values(&new_user)
        .get_result::<User>(&mut conn)
    {
        Ok(inserted_user) => HttpResponse::Created().json(inserted_user),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to insert user: {}", err))
        }
    }
}

pub async fn get_users(state: web::Data<AppState>) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    match users.load::<User>(&mut conn) {
        Ok(results) => HttpResponse::Ok().json(results),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to load users: {}", err))
        }
    }
}

pub async fn get_user(state: web::Data<AppState>, user_id: web::Path<i32>) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    match users.filter(id.eq(*user_id)).first::<User>(&mut conn) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::NotFound().body(format!("User not found: {}", err)),
    }
}

pub async fn update_user(
    state: web::Data<AppState>,
    user_id: web::Path<i32>,
    updated_user: web::Json<NewUser>,
) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    match diesel::update(users.find(*user_id))
        .set(full_name.eq(&updated_user.full_name))
        .execute(&mut conn)
    {
        Ok(_) => HttpResponse::Ok().json(updated_user.into_inner()),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to update user: {}", err))
        }
    }
}

pub async fn delete_user(state: web::Data<AppState>, user_id: web::Path<i32>) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    match diesel::delete(users.find(*user_id)).execute(&mut conn) {
        Ok(_) => HttpResponse::Ok().body(format!("Deleted user with id: {}", user_id)),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to delete user: {}", err))
        }
    }
}
