mod controller;
pub mod global;
pub mod service;

use idea_common::log::init_tk_log;

#[macro_use] extern crate lazy_static;
lazy_static! {

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 日志
    init_tk_log();

    // http server
    let http_server = actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .wrap(actix_web::middleware::Logger::default())
            .wrap(controller::cors())
            .configure(controller::register_routes)
    });
    log::info!("IDEA-AGENT starting at http://0.0.0.0:8080");
    http_server.workers(4).bind(("0.0.0.0", 8080))?.run().await
}