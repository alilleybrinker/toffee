use anyhow::Result;
use serde::Serialize;
use sqlx::sqlite::SqlitePool;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Model {
    pool: Arc<SqlitePool>,
}

impl Model {
    /// Initialize the model by connecting to the database.
    pub async fn new(url: &str) -> Result<Self> {
        let pool = Arc::new(SqlitePool::connect(url).await?);
        let model = Model { pool };
        Ok(model)
    }

    /// Do operations around TODOs.
    pub fn todo(&self) -> TodoHandle<'_> {
        TodoHandle { model: self }
    }
}

pub struct TodoHandle<'m> {
    model: &'m Model,
}

impl<'m> TodoHandle<'m> {
    pub async fn all(&self) -> Result<Vec<Todo>> {
        let mut conn = self.model.pool.acquire().await?;

        let todos = sqlx::query_as!(
            Todo,
            r#"SELECT id, description, done FROM todos ORDER BY id"#
        )
        .fetch_all(&mut *conn)
        .await?;

        Ok(todos)
    }

    /// Create a new TODO
    pub async fn create(&self, desc: &str) -> Result<Todo> {
        let mut conn = self.model.pool.acquire().await?;

        let id = sqlx::query!(r#"INSERT INTO todos ( description ) VALUES ( ?1 )"#, desc)
            .execute(&mut *conn)
            .await?
            .last_insert_rowid();

        let todo = Todo {
            id,
            description: desc.to_owned(),
            done: false,
        };

        Ok(todo)
    }

    /// Get a single TODO by ID.
    pub async fn get(&self, id: i64) -> Result<Todo> {
        let mut conn = self.model.pool.acquire().await?;

        let todos = sqlx::query_as!(
            Todo,
            r#"SELECT id, description, done FROM todos WHERE id = ?1"#,
            id
        )
        .fetch_one(&mut *conn)
        .await?;

        Ok(todos)
    }

    /// Mark a TODO as complete.
    pub async fn mark_complete(&self, id: i64) -> Result<bool> {
        let mut conn = self.model.pool.acquire().await?;

        let rows_affected = sqlx::query!(r#"UPDATE todos SET done = TRUE WHERE id = ?1"#, id)
            .execute(&mut *conn)
            .await?
            .rows_affected();

        Ok(rows_affected > 0)
    }

    /// Mark a TODO as incomplete.
    pub async fn mark_incomplete(&self, id: i64) -> Result<bool> {
        let mut conn = self.model.pool.acquire().await?;

        let rows_affected = sqlx::query!(r#"UPDATE todos SET done = FALSE WHERE id = ?1"#, id)
            .execute(&mut *conn)
            .await?
            .rows_affected();

        Ok(rows_affected > 0)
    }

    /// Delete a TODO.
    pub async fn delete(&self, id: i64) -> Result<bool> {
        let mut conn = self.model.pool.acquire().await?;

        let rows_affected = sqlx::query!(r#"DELETE FROM todos WHERE id = ?1"#, id)
            .execute(&mut *conn)
            .await?
            .rows_affected();

        Ok(rows_affected > 0)
    }
}

/// A single todo from the database.
#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Todo {
    /// The ID of the TODO
    id: i64,
    /// The description of the TODO
    description: String,
    /// Whether the TODO is done
    done: bool,
}
