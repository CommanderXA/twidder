use actix_web::{http::header::ContentType, post, web, HttpResponse};
use entity::user;
use entity::user::Entity as User;
use migration::sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::{models::user::UserLogin, AppState};

#[post("/login")]
pub async fn login(state: web::Data<AppState>, data: web::Json<UserLogin>) -> HttpResponse {
    let conn = &state.db_conn;
    let form = data.into_inner();

    let mut hasher = Sha256::new();
    hasher.update(form.password.as_bytes());
    let hashed_pass = hasher.finalize();

    let mut user: Option<user::Model> = User::find()
        .filter(user::Column::Username.eq(form.username))
        .one(conn)
        .await
        .unwrap();

    if user != None && user.clone().unwrap().password != hex::encode(hashed_pass) {
        user = None;
    }

    match user {
        Some(_) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .body("200"),
        None => HttpResponse::NotFound()
            .content_type(ContentType::json())
            .body("404"),
    }
}

#[post("/register")]
pub async fn register(state: web::Data<AppState>, data: web::Json<user::Model>) -> HttpResponse {
    let conn = &state.db_conn;
    let form = data.into_inner();

    let mut hasher = Sha256::new();
    hasher.update(form.password.as_bytes());
    let hashed_pass = hasher.finalize();

    let result = user::ActiveModel {
        id: Set(Uuid::new_v4()),
        username: Set(form.username),
        email: Set(form.email),
        password: Set(hex::encode(&hashed_pass)),
        first_name: Set(form.first_name),
        last_name: Set(form.last_name),
        ..Default::default()
    }
    .insert(conn)
    .await;

    match result {
        Ok(_) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .body("User created"),
        Err(_) => HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .body("Could not create user. Try again later!"),
    }
}
