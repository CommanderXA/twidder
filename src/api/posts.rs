use actix_web::{delete, get, patch, web, Responder, Result};
use chrono::Utc;

use crate::models::posts::{self, Media};

#[get("/")]
pub async fn index() -> Result<impl Responder> {
    let obj = posts::Post::new(
        "Test".to_owned(),
        "Super Test description".to_owned(),
        Some(Media::new(
            "123123123".to_owned(),
            posts::MediaType::Picture,
        )),
        Utc::now(),
    );
    Ok(web::Json(obj))
}

#[get("")]
pub async fn post() -> Result<impl Responder> {
    let obj = posts::Post::new(
        "Test".to_owned(),
        "Super Test description".to_owned(),
        Some(Media::new(
            "123123123".to_owned(),
            posts::MediaType::Picture,
        )),
        Utc::now(),
    );
    Ok(web::Json(obj))
}

#[patch("/edit")]
pub async fn edit_post() -> Result<impl Responder> {
    Ok(web::Json(200))
}

#[delete("/delete")]
pub async fn delete_post() -> Result<impl Responder> {
    Ok(web::Json(200))
}
