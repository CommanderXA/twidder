use actix_web::web;

use crate::api::{
    posts::{self, delete_post, edit_post, post},
    settings, users,
};

/*
    Config of the Router.
*/

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(web::scope("/users")
                .service(users::index)
                .service(web::scope("/{id}")
                    .service(users::user)
                    .service(users::edit_user)
                    .service(users::delete_user)
                )
            )
            .service(
                web::scope("/posts")
                    .service(posts::index)
                    .service(
                        web::scope("/{id}")
                            .service(post)
                            .service(edit_post)
                            .service(delete_post),
                    ),
            )
            .service(web::scope("/settings")
                .service(settings::index)),
    );
}
