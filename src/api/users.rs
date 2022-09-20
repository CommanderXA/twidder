use std::str::FromStr;

use actix_web::{
    delete, get, http::header::ContentType, patch, web, HttpRequest, HttpResponse, Responder,
    Result,
};

use entity::user;
use entity::user::Entity as User;
use migration::sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Serialize, Deserialize)]
pub struct UserPath {
    id: String,
}

#[get("")]
pub async fn index() -> Result<impl Responder> {
    Ok(web::Json(2))
}

#[get("")]
pub async fn get_user(state: web::Data<AppState>, path: web::Path<UserPath>) -> HttpResponse {
    let conn = &state.db_conn;
    let id: String = path.into_inner().id;

    let user: Option<user::Model> = User::find_by_id(uuid::Uuid::from_str(&id).unwrap())
        .one(conn)
        .await
        .unwrap();

    match user {
        Some(user) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(serde_json::to_string(&user).unwrap()),
        _ => HttpResponse::NotFound()
            .content_type(ContentType::json())
            .body("User not found"),
    }
}

#[patch("")]
pub async fn edit_user(req: HttpRequest) -> Result<impl Responder> {
    let user_id: String = req.match_info().get("id").unwrap().parse().unwrap();
    Ok(web::Json(user_id))
}

#[delete("")]
pub async fn delete_user(req: HttpRequest) -> Result<impl Responder> {
    let user_id: String = req.match_info().get("id").unwrap().parse().unwrap();
    Ok(web::Json(user_id))
}
