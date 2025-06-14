use actix_web::{
    HttpResponse,
    web::{self, ServiceConfig},
};

async fn list_movies() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn get_movie() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn create_movie() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn delete_movie() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn patch_movie() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn service(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/v1/films")
            .route("", web::get().to(list_movies))
            .route("", web::post().to(create_movie))
            .service(
                web::scope("/{film_id}")
                    .route("", web::patch().to(patch_movie))
                    .route("", web::delete().to(delete_movie))
                    .route("", web::get().to(get_movie)),
            ),
    );
}
