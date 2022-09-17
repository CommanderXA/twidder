use actix_web::{get, web, Responder, Result};
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
