use actix_web::{Responder, HttpResponse, web::Json};

mod dynamic_programming;
mod model;

pub async fn handle(body: Json<model::TukarUangRequest>) -> impl Responder {
    let dp_result = dynamic_programming::calculate_dp(body.0.pecahan_koin_tersedia, body.0.uang);

    let resp = model::TukarUangResponse::new(dp_result);
    HttpResponse::Ok().json(resp)
}
