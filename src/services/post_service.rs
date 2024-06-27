use crate::models::post::{NewPost, Post};
use crate::schema::posts::dsl::{body, id, posts, published, title};
use crate::AppState;
use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;

pub async fn create_post(
    state: web::Data<AppState>,
    new_post: web::Json<NewPost>,
) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    let new_post = NewPost {
        title: new_post.title.clone(),
        body: new_post.body.clone(),
        published: new_post.published.clone(),
        subtitle: new_post.subtitle.clone(),
    };

    match diesel::insert_into(posts)
        .values(&new_post)
        .get_result::<Post>(&mut conn)
    {
        Ok(inserted_post) => HttpResponse::Created().json(inserted_post),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to insert post: {}", err))
        }
    }
}

pub async fn get_posts(state: web::Data<AppState>) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    match posts.load::<Post>(&mut conn) {
        Ok(results) => HttpResponse::Ok().json(results),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to load posts: {}", err))
        }
    }
}

pub async fn get_post(state: web::Data<AppState>, post_id: web::Path<i32>) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    match posts.filter(id.eq(*post_id)).first::<Post>(&mut conn) {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(err) => HttpResponse::NotFound().body(format!("Post not found: {}", err)),
    }
}

pub async fn update_post(
    state: web::Data<AppState>,
    post_id: web::Path<i32>,
    updated_post: web::Json<NewPost>,
) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    match diesel::update(posts.find(*post_id))
        .set((
            title.eq(&updated_post.title),
            body.eq(&updated_post.body),
            published.eq(updated_post.published),
        ))
        .execute(&mut conn)
    {
        Ok(_) => HttpResponse::Ok().json(updated_post.into_inner()),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to update post: {}", err))
        }
    }
}

pub async fn delete_post(state: web::Data<AppState>, post_id: web::Path<i32>) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    match diesel::delete(posts.find(*post_id)).execute(&mut conn) {
        Ok(_) => HttpResponse::Ok().body(format!("Deleted post with id: {}", post_id)),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to delete post: {}", err))
        }
    }
}
