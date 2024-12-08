use core::panic;
use std::fs;

use config::{ logger::init_logger, CONFIGURATION };
use internal::{ container::container::new, infra::{ database::migration::migrate, http::server } };

#[actix_web::main]
async fn main() {
    init_logger();

    if let Err(e) = migrate() {
        panic!("{}", e.to_string());
    }

    if let Err(e) = fs::create_dir_all(&CONFIGURATION.file_storage_location) {
        panic!("{}", e.to_string());
    }

    match new() {
        Ok(container) =>
            match server::start_server(container).await {
                Ok(res) => res,
                Err(e) => panic!("{}", e.to_string()),
            }
        Err(e) => panic!("{}", e.to_string()),
    }
}
