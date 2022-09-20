use actix_web::{
    get, http::header::ContentType, patch, post, web, HttpResponse, Responder, Result,
};
use uuid::Uuid;

use crate::models::settings::{Notifications, Settings};

#[get("")]
pub async fn index() -> Result<impl Responder> {
    let obj = Settings::new(
        Uuid::new_v4(),
        "auto".to_owned(),
        "en_US".to_owned(),
        Notifications::new(true, true),
    );
    Ok(web::Json(obj))
}

#[patch("")]
pub async fn edit_settings() -> Result<impl Responder> {
    let obj = Settings::new(
        Uuid::new_v4(),
        "auto".to_owned(),
        "en_US".to_owned(),
        Notifications::new(true, true),
    );
    Ok(web::Json(obj))
}

#[post("")]
pub async fn create_settings() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body("OK")
}
