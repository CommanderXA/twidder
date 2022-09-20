use actix_web::web;

use crate::api::{auth, posts, settings, users};

/*
    Config of the Router.
*/

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/auth")
                    .service(auth::login)
                    .service(auth::register),
            )
            .service(
                web::scope("/users").service(users::index).service(
                    web::scope("/{id}")
                        .service(users::get_user)
                        .service(users::edit_user)
                        .service(users::delete_user),
                ),
            )
            .service(
                web::scope("/posts")
                    .service(posts::index)
                    .service(posts::create_post)
                    .service(
                        web::scope("/{id}")
                            .service(posts::post)
                            .service(posts::edit_post)
                            .service(posts::delete_post),
                    ),
            )
            .service(
                web::scope("/settings")
                    .service(settings::index)
                    .service(settings::create_settings)
                    .service(settings::edit_settings),
            ),
    );
}

/*
    Routes:

    /api/
        - auth
            - /login
            - /register

        - users
            - 
            - /{id}

        - posts/
            - 
            - /{id}

        - settings/
*/
