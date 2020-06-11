use actix;

mod app;

fn main() {
    let sys = actix::System::new("gendon");
    app::start("0.0.0.0:8000".to_string());
    let _ = sys.run();
}
