use actix;

mod app;

fn main() {
    let sys = actix::System::new("gendon");
    app::start();
    let _ = sys.run();
}
