mod app;
mod blocks;
mod core;

fn main() {
    if let Err(e) = app::runner::run() {
        eprintln!("ERROR: {:#}", e);
        std::process::exit(1);
    }
}
