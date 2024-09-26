use dependencies::init_logger;
use internal::{ container::container::{ new, Container }, infra::http::server };

#[actix_web::main]
async fn main() {
    init_logger();
    let container: Container = new();
    let _ = server::start_server(container).await;
}
