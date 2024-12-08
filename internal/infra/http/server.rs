use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{ middleware::Logger, web::JsonConfig, App, HttpServer };

use crate::container::container::Container;

use super::routes;

pub async fn start_server(container: Container) -> std::io::Result<()> {
    HttpServer::new(move || {
        let container_clone = Arc::new(container.clone());
        let logger = Logger::default();
        let cors = Cors::default()
            .allowed_origin("https://*")
            .allowed_origin("http://*")
            .allowed_methods(["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(["Accept", "Authorization", "Content-Type", "X-CSRF-Token"])
            .expose_headers(["Link"])
            .max_age(300);
        return App::new()
            .app_data(JsonConfig::default().limit(4 * 1024 * 1024))
            .wrap(logger)
            .wrap(cors)
            .configure(|cfg| routes::init_routes(cfg, container_clone.clone()));
    })
        .bind(("0.0.0.0", 8080))?
        .run().await
}
