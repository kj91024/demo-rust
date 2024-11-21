use std::env;
use std::io::Error;
use std::sync::Arc;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySqlPool};
use tracing::{error, info};

pub async fn pool() -> Result<Arc<MySqlPool>, Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL debe estar configurado");

    let pool_result = MySqlPoolOptions::new()
        .max_connections(10)
        .idle_timeout(std::time::Duration::from_secs(30))
        .connect(&database_url)
        .await;

    let pool = match pool_result {
        Ok(pool) => {
            info!("Pool de conexiones establecido");
            Arc::new(pool)
        }
        Err(err) => {
            error!("{}", format!("Error al conectar al pool: {}", err));
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "No se pudo conectar al pool"));
        }
    };

    Ok(pool)
}
