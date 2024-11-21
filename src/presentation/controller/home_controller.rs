use std::sync::Arc;
use actix_web::{get, web, HttpRequest, HttpResponse};
use askama::Template;
use serde::Serialize;
use sqlx::{MySqlPool, Row};
use tracing::error;
use crate::presentation::ui::home::index_template::IndexTemplate;

#[get("/")]
pub async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Estoy en el home")
}

#[get("/home")]
pub async fn home() -> HttpResponse {
    let name = "Carlos".to_string();
    let age = 25;
    let items = vec!["aa", "bb", "cc"];

    let template = IndexTemplate { name, age, items };

    let rendered = template.render().unwrap();
    HttpResponse::Ok().body(rendered)
}

#[derive(Serialize)]
struct Asd {
    id: i8,
    age: String,
}
#[get("/asd")]
pub async fn asd(_req: HttpRequest, pool: web::Data<Arc<MySqlPool>>) -> HttpResponse {
    let pool_ref = pool.get_ref().as_ref(); // Esto te da un `&MySqlPool`

    // Hacemos la consulta a la base de datos usando el pool
    let row = sqlx::query("SELECT id, asd FROM test WHERE id = ?")
        .bind(1) // Parámetro de la consulta
        .fetch_one(pool_ref) // Ejecutar la consulta usando el pool
        .await;

    match row {
        Ok(record) => {
            let response = Asd {
                id: record.get::<i8, _>("id"),
                age: record.get::<String, _>("asd"),
            };
            HttpResponse::Ok().json(response)
        }
        Err(err) => {
            error!("Query error: {}", err);
            HttpResponse::InternalServerError().body("SQL query error")
        }
    }

    /*
    let name = "asd";
    let age = 25;
    let response = Asd {name, age};
    HttpResponse::Ok().json(response)
    */
}