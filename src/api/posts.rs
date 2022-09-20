use actix_web::{
    delete, get, http::header::ContentType, patch, post, web, HttpResponse, Responder, Result,
};
use chrono::Utc;

use entity::post as Post;

use crate::{
    models::posts::{self, Media},
    AppState,
};

#[get("")]
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

#[post("")]
pub async fn create_post(
    state: web::Data<AppState>,
    request: web::Json<Post::Model>,
) -> HttpResponse {
    let conn = &state.db_conn;
    let form = request.into_inner();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(format!("Title: {:?},\nText: {:?}", form.title, form.text))
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

#[patch("")]
pub async fn edit_post() -> Result<impl Responder> {
    Ok(web::Json(200))
}

#[delete("")]
pub async fn delete_post() -> Result<impl Responder> {
    Ok(web::Json(200))
}
