use actix_web::{get, web, Responder, Result, patch};
use uuid::Uuid;

use crate::models::settings::{self, Language, Notifications, Settings};

#[get("/")]
pub async fn index() -> Result<impl Responder> {
    let obj = Settings::new(
        Uuid::new_v4(),
        settings::Theme::Auto,
        Language::English,
        Notifications::new(true, true),
    );
    Ok(web::Json(obj))
}

#[patch("/")]
pub async fn edit_settings() -> Result<impl Responder> {
    let obj = Settings::new(
        Uuid::new_v4(),
        settings::Theme::Auto,
        Language::English,
        Notifications::new(true, true),
    );
    Ok(web::Json(obj))
}

