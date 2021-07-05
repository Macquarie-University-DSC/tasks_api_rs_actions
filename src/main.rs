use std::env;
use std::path::Path;

use actix_cors::Cors;
use actix_web::{error::JsonPayloadError, middleware::Logger, web, App, HttpServer};
use anyhow::Result;
use log::info;
use sqlx::{migrate::Migrator, PgPool};

mod errors;
mod handlers;
mod model;

#[cfg(test)]
mod tests;

use errors::{ClientError, ErrorCodes};

#[actix_web::main]
async fn main() -> Result<()> {
    // Start dotenv for development
    #[cfg(debug_assertions)]
    dotenv::dotenv().ok();

    // Start logger
    env_logger::init();

    // Set up database with migrations
    let migrations = Migrator::new(Path::new("./migrations")).await?;
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set");
    let db_pool = PgPool::connect(&database_url).await?;
    migrations.run(&db_pool).await?;

    // Set access point for our server
    let access_point = "127.0.0.1:8080";
    info!("Starting server at: {}", access_point);

    // Starting server
    HttpServer::new(move || {
        let cors = Cors::default().allow_any_origin();

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .data(db_pool.clone())
            .app_data(web::JsonConfig::default().error_handler(|err, _req| {
                let msg = match err {
                    JsonPayloadError::Overflow => "Request is too large.".to_string(),
                    JsonPayloadError::ContentType => {
                        "Cannot process this content, invalid format.".to_string()
                    }
                    JsonPayloadError::Deserialize(_) => "Incorrect Syntax applied".to_string(),
                    JsonPayloadError::Payload(_) => {
                        "Something went wrong communicating with the server.".to_string()
                    }
                };
                ClientError::new(ErrorCodes::InvalidInput, msg).into()
            }))
            .service(handlers::create_task)
            .service(handlers::read_all_tasks)
            .service(handlers::read_task_by_id)
            .service(handlers::update_task)
            .service(handlers::delete_task)
    })
    .bind(access_point)?
    .run()
    .await?;

    Ok(())
}
