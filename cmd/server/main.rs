use core::panic;

use config::logger::init_logger;
use internal::{ container::container::new, infra::{ database::migration::migrate, http::server } };

#[actix_web::main]
async fn main() {
    init_logger();

    if let Err(e) = migrate() {
        panic!("{}", e.to_string());
    }

    match new() {
        Ok(container) => {
            match server::start_server(container).await {
                Ok(res) => res,
                Err(e) => panic!("{}", e.to_string()),
            }
        }
        Err(e) => panic!("{}", e.to_string()),
    }
}
