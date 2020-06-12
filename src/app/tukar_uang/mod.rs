use actix_web::{Responder, HttpResponse, web::Json};

mod dynamic_programming;
mod model;

pub async fn handle(body: Json<model::TukarUangRequest>) -> impl Responder {
    let mut dp_result = dynamic_programming::calculate_dp(body.0.pecahan_koin_tersedia, body.0.uang);
    if dp_result >= dynamic_programming::MAX {
        dp_result = -1;
    }

    let resp = model::TukarUangResponse::new(dp_result);
    HttpResponse::Ok().json(resp)
}
