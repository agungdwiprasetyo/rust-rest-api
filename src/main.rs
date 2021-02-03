use actix;
use dotenv::dotenv;
use std::env;

mod app;

fn main() {
    dotenv().ok();

    let port = env::var("PORT");
    let sys = actix::System::new("gendon");
    app::start(port.unwrap());
    let _ = sys.run();
}
