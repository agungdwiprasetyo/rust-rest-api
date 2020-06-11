use actix_web::{HttpServer, App, web};

mod index;

pub fn start() {
    let address = "0.0.0.0:8000";
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
}
