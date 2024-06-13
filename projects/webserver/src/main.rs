use actix_files as fs;
use actix_web::{middleware, web, App, HttpServer};
use dotenvy::dotenv;
use std::env;

mod pages;
use general;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv().expect(".env file not found");
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let address = "0.0.0.0";
    let port = env::var("SUNBERRY_WEBSERVER_PORT").expect("SUNBERRY_WEBSERVER_PORT not set in environment.").parse::<u16>().unwrap();

    log::info!("{}", general::separator());
    log::info!("Starting Webserver at {}:{}", address, port);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/").route(web::get().to(pages::home::home)))
            .service(web::resource("/systeminfo").route(web::get().to(pages::systeminfo::page_systeminfo)))
            .service(web::resource("/{any}").route(web::get().to(pages::mdpage::subpage)))
            .service(fs::Files::new("/static", "static").show_files_listing())
            .service(fs::Files::new("/", "static/favicon").show_files_listing())
    })
    .workers(4)
    .bind((address, port))?
    .run()
    .await
}