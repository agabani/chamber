use actix_web::{web, HttpResponse};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("liveness", web::get().to(liveness_get))
        .route("readiness", web::get().to(readiness_get));
}

#[allow(clippy::unused_async)]
async fn liveness_get() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[allow(clippy::unused_async)]
async fn readiness_get() -> HttpResponse {
    HttpResponse::Ok().finish()
}
