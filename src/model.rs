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
    #[cfg(test)]
    pub fn create_test_data() -> Vec<NewTask> {
        vec![
            NewTask {
                name: "Test Task 1".to_string(),
                description: "Test Task 1 description".to_string(),
                due_date: None,
                is_complete: false,
            },
            NewTask {
                name: "Test Task 2".to_string(),
                description: "Test Task 2 description".to_string(),
                due_date: None,
                is_complete: true,
            },
            NewTask {
                name: "Test Task 3".to_string(),
                description: "Test Task 3 description".to_string(),
                due_date: Some(1625443200000),
                is_complete: false,
            },
            NewTask {
                name: "Test Task 4".to_string(),
                description: "Test Task 4 description".to_string(),
                due_date: Some(1625443200000),
                is_complete: true,
            },
            NewTask {
                name: "Test Task 5".to_string(),
                description: "Test Task 5 description".to_string(),
                due_date: Some(1625443200000),
                is_complete: true,
            }
        ]
    }

    #[cfg(test)]
    pub fn create_result_data() -> Vec<Task> {
        vec![
            Task {
                id: 1,
                name: "Test Task 1".to_string(),
                description: "Test Task 1 description".to_string(),
                due_date: None,
                is_complete: false,
            },
            Task {
                id: 2,
                name: "Test Task 2".to_string(),
                description: "Test Task 2 description".to_string(),
                due_date: None,
                is_complete: true,
            },
            Task {
                id: 3,
                name: "Test Task 3".to_string(),
                description: "Test Task 3 description".to_string(),
                due_date: Some(1625443200000),
                is_complete: false,
            },
            Task {
                id: 4,
                name: "Test Task 4".to_string(),
                description: "Test Task 4 description".to_string(),
                due_date: Some(1625443200000),
                is_complete: true,
            },
            Task {
                id: 5,
                name: "Test Task 5".to_string(),
                description: "Test Task 5 description".to_string(),
                due_date: Some(1625443200000),
                is_complete: true,
            }
        ]
    }

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
