use actix_web::{HttpRequest, Responder, HttpResponse};

mod model;

pub async fn get(_req: HttpRequest) -> impl Responder {
    let query = _req.query_string();
    let v: Vec<&str> = query.split('=').collect();
    println!("{:?}", v);
    
    let resp = model::construct("Hello world!".to_string());
    HttpResponse::Ok().json(resp)
}