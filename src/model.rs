use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;
use sqlx::FromRow;
use sqlx::Result;

#[derive(Debug, PartialEq, Serialize, Deserialize, FromRow)]
pub struct Task {
    id: i32,
    name: String,
    description: String,
    due_date: Option<i64>,
    is_complete: bool,
}

#[derive(Deserialize)]
pub struct NewTask {
    name: String,
    description: String,
    due_date: Option<i64>,
    #[serde(default)]
    is_complete: bool,
}

impl Task {
    pub async fn create(task: &NewTask, pool: &PgPool) -> Result<Task> {
        let result = sqlx::query_as!(
            Task,
            "
            INSERT INTO tasks (name, description, due_date, is_complete)
                VALUES ($1, $2, $3, $4)
                RETURNING id, name, description, due_date, is_complete
            ",
            &task.name,
            &task.description,
            task.due_date,
            task.is_complete
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn read_all(pool: &PgPool) -> Result<Vec<Task>> {
        let result = sqlx::query_as!(Task, "SELECT * FROM tasks")
            .fetch_all(pool)
            .await?;

        Ok(result)
    }

    pub async fn read_by_id(id: i32, pool: &PgPool) -> Result<Task> {
        let result = sqlx::query_as!(Task, "SELECT * FROM tasks WHERE id = $1", id)
            .fetch_one(pool)
            .await?;

        Ok(result)
    }

    pub async fn update(id: i32, task: &NewTask, pool: &PgPool) -> Result<Task> {
        let result = sqlx::query_as!(
            Task,
            "
            UPDATE tasks
                SET name = $2, description = $3, due_date = $4, is_complete = $5
                WHERE id = $1
                RETURNING id, name, description, due_date, is_complete
            ",
            id,
            &task.name,
            &task.description,
            task.due_date,
            task.is_complete
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn delete(id: i32, pool: &PgPool) -> Result<()> {
        sqlx::query!("DELETE FROM tasks WHERE id = $1", id)
            .execute(pool)
            .await?;

        Ok(())
    }
}
