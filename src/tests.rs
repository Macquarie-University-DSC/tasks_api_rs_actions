use std::env;
use std::path::Path;

use crate::model::*;
use anyhow::Result;
use sqlx::migrate::Migrator;
use sqlx::PgPool;

#[async_std::test]
async fn database_tests() -> Result<()> {
    let migrator = Migrator::new(Path::new("./migrations")).await?;
    let conn_str = env::var("DATABASE_URL")?;
    let pool = PgPool::connect(&conn_str).await?;
    migrator.run(&pool).await?;

    let tasks_to_test = Task::create_test_data();
    let expected = Task::create_result_data();

    for (test_task, expected_task) in tasks_to_test.iter().zip(expected.iter()) {
        let result = Task::create(&test_task, &pool).await?;

        assert_eq!(result, *expected_task);
    }

    let result = Task::read_all(&pool).await?;
    assert_eq!(result, expected);

    Ok(())
}
