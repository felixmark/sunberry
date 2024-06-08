use actix_files as fs;
use actix_web::{middleware, web, App, HttpServer};

mod pages;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let address = "0.0.0.0";
    let port = 8080;

    log::info!("Starting Webserver at {}:{}", address, port);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/").route(web::get().to(pages::index::page_index)))
            .service(web::resource("/stats").route(web::get().to(pages::stats::page_stats)))
            .service(web::resource("/links").route(web::get().to(pages::links::page_links)))
            .service(web::resource("/notes").route(web::get().to(pages::notes::page_notes)))
            .service(fs::Files::new("/static", "static").show_files_listing())
            .service(fs::Files::new("/", "static/favicon").show_files_listing())
    })
    .bind((address, port))?
    .run()
    .await
}