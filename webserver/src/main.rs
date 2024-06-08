use actix_files as fs;
use actix_web::{middleware, web, App, HttpServer};

mod index;
mod stats;
mod links;
mod notes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/").route(web::get().to(index::page_index)))
            .service(web::resource("/stats").route(web::get().to(stats::page_stats)))
            .service(web::resource("/links").route(web::get().to(links::page_links)))
            .service(web::resource("/notes").route(web::get().to(notes::page_notes)))
            .service(fs::Files::new("/static", "static").show_files_listing())
            .service(fs::Files::new("/", "static/favicon").show_files_listing())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}