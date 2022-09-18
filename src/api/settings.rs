use actix_web::{get, patch, web, Responder, Result};
use uuid::Uuid;

use crate::models::settings::{Notifications, Settings};

#[get("/")]
pub async fn index() -> Result<impl Responder> {
    let obj = Settings::new(
        Uuid::new_v4(),
        "auto".to_owned(),
        "en_US".to_owned(),
        Notifications::new(true, true),
    );
    Ok(web::Json(obj))
}

#[patch("/")]
pub async fn edit_settings() -> Result<impl Responder> {
    let obj = Settings::new(
        Uuid::new_v4(),
        "auto".to_owned(),
        "en_US".to_owned(),
        Notifications::new(true, true),
    );
    Ok(web::Json(obj))
}
