use lokker::Logger;

#[test]
fn main() {
    Logger::init().unwrap();

    log::info!("Hello, World!");
}
