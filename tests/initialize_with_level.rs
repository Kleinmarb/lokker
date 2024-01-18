use lokker::Logger;

#[test]
fn main() {
    Logger::init_with_level(log::LevelFilter::Info).unwrap();

    log::info!("Hello, World!");
}
