mod budget;
mod database;
use database::{ JsonDatabase };
mod handlers;

use actix_web::{web, App, HttpServer, Responder, HttpResponse, HttpRequest};
use actix_web::dev::Service;
use actix_files;
use std::fs;
use mime_guess::from_path;

use rust_embed::{RustEmbed};

#[derive(RustEmbed)]
#[folder = "./frontend/build"]
struct Assets;

fn handle_embedded_file(path: &str) -> HttpResponse {
    match Assets::get(path) {
        Some(content) => HttpResponse::Ok()
            .content_type(from_path(path).first_or_octet_stream().as_ref())
            .body(content.data.into_owned()),
        None => HttpResponse::NotFound().body("404 Not Found")
    }
}

pub async fn root_handler() -> impl Responder {
    handle_embedded_file("index.html")
}

pub async fn asset_handler(req: HttpRequest) -> impl Responder {
    println!("FAEADASDASDASDASDASD");
    println!("{:?}", req.match_info());
    match req.match_info().get("file") {
        Some(s) => {
            println!("{s}");
            handle_embedded_file(s)
        },
        None => handle_embedded_file("index.html")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let db = JsonDatabase::from_file("mydb.json").unwrap();

    for d in Assets::iter() {
        println!("{d}");
    }

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
                .route(web::get().to(root_handler)))
            .service(web::resource("/api/addEntry").
                route(web::post().to(handlers::add_entry_handler)))
            .service(web::resource("/api/deleteEntry")
                .route(web::post().to(handlers::delete_entry_handler)))
            .service(web::resource("/api/updateEntry")
                .route(web::post().to(handlers::update_entry_handler)))
            .service(web::resource("/api/getById")
                .route(web::post().to(handlers::get_by_id_handler)))
            .service(web::resource("/api/getByTags")
                .route(web::post().to(handlers::get_by_tags_handler)))
            .service(web::resource("/api/getByMonth")
                .route(web::post().to(handlers::get_by_month_handler)))
            .service(web::resource("/api/getByAdvanced")
                .route(web::post().to(handlers::get_by_advanced_handler)))
            .service(web::resource("/{file:.*}")
                .route(web::get().to(asset_handler)))
            //.service(actix_files::Files::new("/assets", "./frontend")
            //    .show_files_listing())
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
