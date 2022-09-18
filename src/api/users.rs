use actix_web::{delete, get, patch, web, HttpRequest, Responder, Result};

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
pub async fn user(req: HttpRequest) -> Result<impl Responder> {
    let user_id: String = req.match_info().get("id").unwrap().parse().unwrap();
    let obj = User::new(
        "username, id: ".to_owned() + &user_id,
        "first_name".to_owned(),
        "last_name".to_owned(),
    );
    Ok(web::Json(obj))
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
