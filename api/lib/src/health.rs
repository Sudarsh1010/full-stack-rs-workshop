use actix_web::{HttpResponse, get};

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[get("version")]
async fn version(db: actix_web::web::Data<sqlx::PgPool>) -> String {
    let result: Result<String, sqlx::Error> = sqlx::query_scalar("select version()")
        .fetch_one(db.get_ref())
        .await;

    match result {
        Ok(version) => version,
        Err(e) => format!("Error: {:?}", e),
    }
}

#[get("/health")]
async fn health() -> HttpResponse {
    HttpResponse::Ok()
        .append_header(("version", "0.0.1"))
        .finish()
}
