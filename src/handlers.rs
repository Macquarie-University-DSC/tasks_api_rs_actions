use actix_web::{delete, get, post, put, web, HttpResponse};
use log::info;
use sqlx::error::Error;
use sqlx::postgres::PgPool;

use crate::errors::{ClientError, ErrorCodes};
use crate::model::{NewTask, Task};

#[post("/task")]
pub async fn create_task(
    new_task: web::Json<NewTask>,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse, ClientError> {
    let result = Task::create(&new_task.into_inner(), db_pool.get_ref())
        .await
        .map_err(|e| {
            info!("database error: {}", e);
            match e {
                Error::Database(_) => ClientError::new(
                    ErrorCodes::InvalidInput,
                    "Database said invalid input.".to_string(),
                ),
                _ => ClientError::new(
                    ErrorCodes::InternalServerError,
                    "Something went wrong with the database.".to_string(),
                ),
            }
        })?;

    Ok(HttpResponse::Created().json(result))
}

#[get("/tasks")]
pub async fn read_all_tasks(db_pool: web::Data<PgPool>) -> Result<HttpResponse, ClientError> {
    let result = Task::read_all(db_pool.get_ref()).await.map_err(|e| {
        info!("database error: {}", e);
        ClientError::new(
            ErrorCodes::InternalServerError,
            "Something went wrong with the database.".to_string(),
        )
    })?;

    Ok(HttpResponse::Ok().json(result))
}

#[get("/task/{id}")]
pub async fn read_task_by_id(
    id: web::Path<i32>,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse, ClientError> {
    let result = Task::read_by_id(id.into_inner(), db_pool.get_ref())
        .await
        .map_err(|e| {
            info!("database error: {}", e);
            match e {
                Error::RowNotFound => ClientError::new(
                    ErrorCodes::InvalidId,
                    "Incorrect id specified, could not find task".to_string(),
                ),
                _ => ClientError::new(
                    ErrorCodes::InternalServerError,
                    "Something went wrong with the database".to_string(),
                ),
            }
        })?;

    Ok(HttpResponse::Ok().json(result))
}

#[put("task/{id}")]
pub async fn update_task(
    id: web::Path<i32>,
    new_task: web::Json<NewTask>,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse, ClientError> {
    let result = Task::update(id.into_inner(), &new_task.into_inner(), db_pool.get_ref())
        .await
        .map_err(|e| {
            info!("database error: {}", e);
            match e {
                Error::RowNotFound => ClientError::new(
                    ErrorCodes::InvalidId,
                    "Incorrect id specified, could not find task".to_string(),
                ),
                _ => ClientError::new(
                    ErrorCodes::InternalServerError,
                    "Something went wrong with the database".to_string(),
                ),
            }
        })?;

    Ok(HttpResponse::Ok().json(result))
}

#[delete("/todo/{id}")]
pub async fn delete_task(
    id: web::Path<i32>,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse, ClientError> {
    Task::delete(id.into_inner(), db_pool.get_ref())
        .await
        .map_err(|e| {
            info!("database error: {}", e);
            match e {
                Error::RowNotFound => ClientError::new(
                    ErrorCodes::InvalidId,
                    "Incorrect id specified, could not find task".to_string(),
                ),
                _ => ClientError::new(
                    ErrorCodes::InternalServerError,
                    "Something went wrong with the database".to_string(),
                ),
            }
        })?;

    Ok(HttpResponse::NoContent().finish())
}
