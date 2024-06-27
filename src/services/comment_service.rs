use crate::models::comment::{Comment, Content, NewComment};
use crate::schema::comments::dsl::{id, comments, content, post_id, user_id};
use crate::AppState;
use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;

pub async fn create_comment(
    state: web::Data<AppState>,
    new_content: web::Json<Content>,
    path: web::Path<(i32, i32)>,
) -> impl Responder {
    let (id_post, id_user) = path.into_inner();
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    let new_comment = NewComment {
        content: new_content.content.clone(),
        user_id: id_user,
        post_id: id_post,
    };

    match diesel::insert_into(comments)
        .values(&new_comment)
        .get_result::<Comment>(&mut conn)
    {
        Ok(inserted_comment) => HttpResponse::Created().json(inserted_comment),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to insert comment: {}", err))
        }
    }
}

pub async fn get_comments(state: web::Data<AppState>) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    match comments.load::<Comment>(&mut conn) {
        Ok(results) => HttpResponse::Ok().json(results),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to load comments: {}", err))
        }
    }
}

pub async fn get_comment(state: web::Data<AppState>, comment_id: web::Path<i32>) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    match comments.filter(id.eq(*comment_id)).load::<Comment>(&mut conn) {
        Ok(result) => {
            if result.is_empty() {
                HttpResponse::NotFound().body("Comment not found.")
            } else {
                HttpResponse::Ok().json(result)
            }
        },
        Err(err) => HttpResponse::InternalServerError().body(format!("Erreur lors du chargement des commentaires : {}", err)),
    }
}

pub async fn get_comments_for_post(
    state: web::Data<AppState>,
    id_post: web::Path<i32>,
) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    match comments.filter(post_id.eq(*id_post)).load::<Comment>(&mut conn) {
        Ok(result) => {
            if result.is_empty() {
                HttpResponse::NotFound().body("Aucun commentaire trouvé pour ce post.")
            } else {
                HttpResponse::Ok().json(result)
            }
        },
        Err(err) => HttpResponse::InternalServerError().body(format!("Erreur lors du chargement des commentaires : {}", err)),
    }
}

pub async fn get_comments_for_user(
    state: web::Data<AppState>,
    id_user: web::Path<i32>,
) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    match comments.filter(user_id.eq(*id_user)).load::<Comment>(&mut conn) {
        Ok(result) => {
            if result.is_empty() {
                HttpResponse::NotFound().body("Aucun commentaire trouvé pour cet utilisateur.")
            } else {
                HttpResponse::Ok().json(result)
            }
        },
        Err(err) => HttpResponse::InternalServerError().body(format!("Erreur lors du chargement des commentaires : {}", err)),
    }
}

pub async fn update_comment(
    state: web::Data<AppState>,
    comment_id: web::Path<i32>,
    updated_comment: web::Json<Content>,
) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    match comments.find(*comment_id).first::<Comment>(&mut conn) {
        Ok(_) => {
            match diesel::update(comments.find(*comment_id))
                .set(content.eq(&updated_comment.content))
                .execute(&mut conn) {
                Ok(_) => HttpResponse::Ok().json(updated_comment.into_inner()),
                Err(err) => HttpResponse::InternalServerError().body(format!("Échec de la mise à jour du commentaire : {}", err)),
            }
        },
        Err(_) => HttpResponse::NotFound().body(format!("Erreur : Le commentaire avec l'id : {} n'existe pas", comment_id)),
    }
}

pub async fn delete_comment(state: web::Data<AppState>, comment_id: web::Path<i32>) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    match comments.find(*comment_id).first::<Comment>(&mut conn) {
        Ok(_) => {
            match diesel::delete(comments.find(*comment_id)).execute(&mut conn) {
                Ok(_) => HttpResponse::Ok().body(format!("Commentaire supprimé avec l'id : {}", comment_id)),
                Err(err) => HttpResponse::InternalServerError().body(format!("Échec de la suppression du commentaire : {}", err)),
            }
        },
        Err(_) => HttpResponse::NotFound().body(format!("Erreur : Le commentaire avec l'id : {} n'existe pas", comment_id)),
    }
}