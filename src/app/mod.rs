use actix_web::{HttpServer, App, web};

mod index;
mod tukar_uang;

pub fn start(address: String) {
    HttpServer::new(|| {
        App::new().configure(routes)
    })
    .bind(&address)
    .unwrap_or_else(|_| panic!("Failed start server in address {}", &address))
    .run();
    
    println!("Server run at address {}", &address);
}

fn routes(app: &mut web::ServiceConfig) {
    app.route("/", web::get().to(index::get));
    app.route("/tukar_uang", web::post().to(tukar_uang::handle));
}
