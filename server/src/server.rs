use crate::{
    error::{AppError, AppResult},
    model::Model,
};
use anyhow::Result;
use axum::{
    extract::{Query, State},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;
use std::ops::Not as _;
use tokio::net::TcpListener;
use tracing::instrument;

pub async fn start_server(port: u16) -> Result<()> {
    let host = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(&host).await?;
    let app = get_app().await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn get_app() -> Result<Router> {
    let url = std::env::var("DATABASE_URL")?;
    let state = Model::new(&url).await?;

    let app = Router::new()
        .route("/api/healthcheck", get(healthcheck_handler))
        .route(
            "/api/todos",
            post(create_todo_handler).get(list_todos_handler),
        )
        .route(
            "/api/todos/:id",
            get(get_todo_handler)
                .patch(edit_todo_handler)
                .delete(delete_todo_handler),
        )
        .with_state(state);

    Ok(app)
}

// GET /api/healthcheck
#[instrument]
#[axum::debug_handler]
async fn healthcheck_handler() -> AppResult {
    Ok(Json(json!({
        "status": "success",
        "message": "A simple todo list app",
    })))
}

// POST /api/todos
#[instrument]
#[axum::debug_handler]
async fn create_todo_handler(
    State(model): State<Model>,
    Json(args): Json<CreateTodoArgs>,
) -> AppResult {
    let todo = model.todo().create(&args.description).await?;
    Ok(Json(json!({ "todo": todo })))
}

#[derive(Debug, Deserialize)]
struct CreateTodoArgs {
    description: String,
}

// GET /api/todos
#[instrument]
#[axum::debug_handler]
async fn list_todos_handler(State(model): State<Model>) -> AppResult {
    let todos = model.todo().all().await?;
    Ok(Json(json!({ "todos": todos })))
}

// GET /api/todos/:id
#[instrument]
#[axum::debug_handler]
async fn get_todo_handler(State(model): State<Model>, Query(query): Query<TodoId>) -> AppResult {
    let todo = model.todo().get(query.id).await?;
    Ok(Json(json!({ "todo": todo })))
}

// PATCH /api/todos/:id
#[instrument]
#[axum::debug_handler]
async fn edit_todo_handler(
    State(model): State<Model>,
    Query(query): Query<TodoId>,
    Json(args): Json<EditTodoArgs>,
) -> AppResult {
    let succeeded = if args.done {
        model.todo().mark_complete(query.id).await?
    } else {
        model.todo().mark_incomplete(query.id).await?
    };

    if succeeded.not() {
        return Err(AppError::not_found(query.id));
    }

    Ok(Json(json!(null)))
}

#[derive(Debug, Deserialize)]
struct EditTodoArgs {
    done: bool,
}

// DELETE /api/todos/:id
#[instrument]
#[axum::debug_handler]
async fn delete_todo_handler(State(model): State<Model>, Query(query): Query<TodoId>) -> AppResult {
    let succeeded = model.todo().delete(query.id).await?;

    if succeeded.not() {
        return Err(AppError::not_found(query.id));
    }

    Ok(Json(json!(null)))
}

#[derive(Debug, Deserialize)]
struct TodoId {
    id: i64,
}
