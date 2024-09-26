use std::sync::Arc;

use actix_web::{ middleware::Logger, App, HttpServer };

use crate::container::container::Container;

use super::routes;

pub async fn start_server(container: Container) -> std::io::Result<()> {
    HttpServer::new(move || {
        let container_clone = Arc::new(container.clone());
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .configure(|cfg| routes::init_routes(cfg, container_clone.clone()))
    })
        .bind(("127.0.0.1", 8080))?
        .run().await
}
