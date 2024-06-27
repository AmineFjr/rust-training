use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use r2d2::Pool;
use services::comment_service::{get_comments_for_post, get_comments_for_user, create_comment, get_comment, get_comments, update_comment, delete_comment};
use services::reservation_service::{get_reservations, create_reservation, get_reservation, update_reservation, delete_reservation};
use services::customer_service::{get_customers, create_customer, get_customer, update_customer, delete_customer};
use services::restaurant_service::{get_restaurants, create_restaurant, get_restaurant, delete_restaurant};
use services::post_service::{create_post, delete_post, get_post, get_posts, update_post};
use services::user_service::{create_user, delete_user, get_user, get_users, update_user};
use services::table_service::{get_tables, create_table, get_table, delete_table};
use std::env;

mod models;
mod schema;
mod services;

#[derive(Clone)]
struct AppState {
    conn: Pool<ConnectionManager<PgConnection>>,
}

pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;

fn get_pool() -> PostgresPool {
    dotenv::dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mgr = ConnectionManager::<PgConnection>::new(url);
    r2d2::Pool::builder()
        .build(mgr)
        .expect("could not build connection pool")
}

fn logging_setup() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    logging_setup();
    let pool = get_pool();
    let state = AppState { conn: pool };

    println!("Backend launched!");
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(state.clone()))
            .service(
                web::scope("/posts")
                    .route("", web::post().to(create_post))
                    .route("", web::get().to(get_posts))
                    .route("/{id}", web::get().to(get_post))
                    .route("/{id}", web::put().to(update_post))
                    .route("/{id}", web::delete().to(delete_post))
                    .route("/{id}/comments", web::get().to(get_comments_for_post))
                    .route("/{post_id}/{user_id}/comments", web::post().to(create_comment))
            )
            .service(
                web::scope("/users")
                    .route("", web::post().to(create_user))
                    .route("", web::get().to(get_users))
                    .route("/{id}", web::get().to(get_user))
                    .route("/{id}", web::put().to(update_user))
                    .route("/{id}", web::delete().to(delete_user))
                    .route("/{id}/comments", web::get().to(get_comments_for_user)),
            )
            .service(
                web::scope("/comments")
                    .route("", web::get().to(get_comments))
                    .route("/{id}", web::get().to(get_comment))
                    .route("/{id}", web::put().to(update_comment))
                    .route("/{id}", web::delete().to(delete_comment)),
            )
            .service(
                web::scope("/customers")
                    .route("", web::get().to(get_customers))
                    .route("", web::post().to(create_customer))
                    .route("/{id}", web::get().to(get_customer))
                    .route("/{id}", web::put().to(update_customer))
                    .route("/{id}", web::delete().to(delete_customer))
            )
            .service(
                web::scope("/tables")
                    .route("", web::get().to(get_tables))
                    .route("", web::post().to(create_table))
                    .route("/{id}", web::get().to(get_table))
                    .route("/{id}", web::delete().to(delete_table))
            )
            .service(
                web::scope("/restaurants")
                    .route("", web::get().to(get_restaurants))
                    .route("/{id}", web::get().to(get_restaurant))
                    .route("", web::post().to(create_restaurant))
                    .route("/{id}", web::delete().to(delete_restaurant)),
            )
            .service(
                web::scope("/reservations")
                    .route("", web::get().to(get_reservations))
                    .route("", web::post().to(create_reservation))
                    .route("/{id_table}/{id_customer}", web::get().to(get_reservation))
                    .route("/{id_table}/{id_customer}", web::put().to(update_reservation))
                    .route("/{id_table}/{id_customer}", web::delete().to(delete_reservation))
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}