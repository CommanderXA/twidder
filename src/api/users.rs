use actix_web::{delete, get, patch, web, Responder, Result};

use crate::models::user::User;

#[get("/")]
pub async fn index() -> Result<impl Responder> {
    let obj = User::new(
        "username".to_owned(),
        "first_name".to_owned(),
        "last_name".to_owned(),
    );
    Ok(web::Json(obj))
}

#[get("")]
pub async fn user() -> Result<impl Responder> {
    let obj = User::new(
        "username".to_owned(),
        "first_name".to_owned(),
        "last_name".to_owned(),
    );
    Ok(web::Json(obj))
}

#[patch("/edit")]
pub async fn edit_user() -> Result<impl Responder> {
    Ok(web::Json(1))
}

#[delete("/delete")]
pub async fn delete_user() -> Result<impl Responder> {
    Ok(web::Json(1))
}
