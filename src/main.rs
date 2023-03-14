mod budget;
mod database;
use database::{ JsonDatabase };
mod handlers;

use actix_web::{web, App, HttpServer};
use actix_web::dev::Service;
use actix_files;
use std::fs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let db = JsonDatabase::from_file("mydb.json").unwrap();

    let port = 9090;
    println!("Server started on port {}", port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .wrap_fn(|req, srv| {
                println!("{} {} from {}", req.method(), req.path(),
                    req.connection_info().peer_addr().unwrap_or("N/A"));
                srv.call(req)
            }) 
            .service(web::resource("/")
                .route(web::get().to(handlers::root_handler)))
            .service(web::resource("/addEntry").
                route(web::post().to(handlers::add_entry_handler)))
            .service(web::resource("/deleteEntry")
                .route(web::post().to(handlers::delete_entry_handler)))
            .service(web::resource("/updateEntry")
                .route(web::post().to(handlers::update_entry_handler)))
            .service(web::resource("/getById")
                .route(web::post().to(handlers::get_by_id_handler)))
            .service(web::resource("/getByTags")
                .route(web::post().to(handlers::get_by_tags_handler)))
            .service(web::resource("/getByMonth")
                .route(web::post().to(handlers::get_by_month_handler)))
            .service(web::resource("/getByAdvanced")
                .route(web::post().to(handlers::get_by_advanced_handler)))
            .service(actix_files::Files::new("/assets", "./frontend")
                .show_files_listing())
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
