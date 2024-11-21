use actix_cors::Cors;

use std::env;
use std::sync::Arc;
use actix_web::{web, App, HttpServer};
use actix_web::middleware::Compress;
use sqlx::{MySqlPool};
use crate::presentation::controller::home_controller;

pub async fn run(pool: Arc<MySqlPool>) -> std::io::Result<()> {
    let server_address = env::var("SERVER_ADDRESS").expect("SERVER_ADDRESS debe estar configurado");
    let server_port = env::var("SERVER_PORT").expect("SERVER_PORT debe estar configurado").parse::<u16>().unwrap();

    let server = HttpServer::new(move || {
        let pool = Arc::clone(&pool); // Clonamos el Arc para compartirlo entre hilos
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000");
        App::new()
            // Compartir el pool de conexiones correctamente usando `web::Data`
            .app_data(web::Data::new(pool))
            //.app_data(web::Data::new(AppState { db: pool.clone() }))
            .wrap(Compress::default())
            .service(home_controller::index)
            .service(home_controller::home)
            .service(home_controller::asd) // Asegúrate de registrar el handler `asd`
            .wrap(cors)
    })
        .bind((server_address.clone().as_str(), server_port))?
        .run();

    server.await
}