fn main() {
    // Initialize logging
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap_or(());
    log::debug!["Hello2"];
    println!("Hello, world!");
}
