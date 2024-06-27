use crate::models::customer::{Customer, NewCustomer};
use crate::schema::customers::dsl::{id, customers, name, email};
use crate::AppState;
use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;

pub async fn create_customer(
    state: web::Data<AppState>,
    new_customer: web::Json<NewCustomer>,
) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    let new_customer = NewCustomer {
        name: new_customer.name.clone(),
        email: new_customer.email.clone(),
    };

    match diesel::insert_into(customers)
        .values(&new_customer)
        .get_result::<Customer>(&mut conn)
    {
        Ok(customer) => HttpResponse::Created().json(customer),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to create customer: {}", err))
        }
    }
}

pub async fn get_customers(state: web::Data<AppState>) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    match customers.load::<Customer>(&mut conn) {
        Ok(results) => HttpResponse::Ok().json(results),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to load customers: {}", err))
        }
    }
}

pub async fn get_customer(state: web::Data<AppState>, customer_id: web::Path<i32>) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    match customers.filter(id.eq(*customer_id)).load::<Customer>(&mut conn) {
        Ok(result) => {
            if result.is_empty() {
                HttpResponse::NotFound().body("Customer not found.")
            } else {
                HttpResponse::Ok().json(result)
            }
        },
        Err(err) => HttpResponse::InternalServerError().body(format!("Erreur lors du chargement des customers : {}", err)),
    }
}

pub async fn update_customer(
    state: web::Data<AppState>,
    customer_id: web::Path<i32>,
    updated_customer: web::Json<NewCustomer>,
) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    let updated_customer = NewCustomer {
        name: updated_customer.name.clone(),
        email: updated_customer.email.clone(),
    };

    match customers.find(*customer_id).first::<Customer>(&mut conn) {
        Ok(_) => {
            match diesel::update(customers.find(*customer_id))
                .set((
                    name.eq(&updated_customer.name),
                    email.eq(&updated_customer.email),
                ))
                .get_result::<Customer>(&mut conn)
                {
                Ok(customer) => HttpResponse::Ok().json(customer),
                Err(err) => {
                    HttpResponse::InternalServerError().body(format!("Failed to update customer: {}", err))
                }
            }
        },
        Err(_) => HttpResponse::NotFound().body(format!("Erreur : Le commentaire avec l'id : {} n'existe pas", customer_id)),
    }
}

pub async fn delete_customer(state: web::Data<AppState>, customer_id: web::Path<i32>) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    match customers.find(*customer_id).first::<Customer>(&mut conn) {
        Ok(_) => {
            match diesel::delete(customers.find(*customer_id)).execute(&mut conn) {
                Ok(_) => HttpResponse::Ok().finish(),
                Err(err) => {
                    HttpResponse::InternalServerError().body(format!("Failed to delete customer: {}", err))
                }
            }
        },
        Err(err) => HttpResponse::NotFound().body(format!("Erreur : Le commentaire avec l'id : {} n'existe pas", err)),
    }
}