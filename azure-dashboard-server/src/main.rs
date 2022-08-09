#![allow(dead_code)]

use crate::settings::Settings;

mod settings;

fn main() {
    // Initialize logging
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap_or(());
    log::debug!(" - loading the configuration file");
    let settings = Settings::new().unwrap();
    log::debug!(" - loading settings = {:?}", settings);

    println!("Hello, world!");
}
