use actix_web::{web, HttpResponse};
use chamber_api_contract::v1::add_one::AddOneResponse;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/add_one/{number}", web::get().to(ping_get));
}

#[allow(clippy::unused_async)]
async fn ping_get(path: web::Path<i64>) -> HttpResponse {
    HttpResponse::Ok().json(AddOneResponse {
        data: path.as_ref() + 1,
    })
}
