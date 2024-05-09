use axum::{
    extract::{Path, State},
    routing::{delete, get, post},
    Json, Router,
};
use db::{
    deadpool_diesel::postgres::Pool,
    diesel::{self, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper},
    models::{NewTodo, Todo},
    schema::todos,
};
use error::Result;
use tokio::net::TcpListener;

mod config;
mod error;

#[tokio::main]
async fn main() -> Result<()> {
    let config = config::Config::new();

    let pool = db::create_pool(&config.database_url);

    let app = Router::new()
        .route("/", get(list_todo))
        .route("/create", post(create_todo))
        .route("/delete/:id", delete(delete_todo))
        .route("/update", post(update_todo))
        .with_state(pool)
        .layer(tower_http::cors::CorsLayer::very_permissive());

    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    println!("Listening on: {}", listener.local_addr()?);

    Ok(axum::serve(listener, app.into_make_service()).await?)
}

async fn list_todo(State(pool): State<Pool>) -> Result<Json<Vec<Todo>>> {
    let conn = pool.get().await?;
    let todos = conn
        .interact(|conn| todos::table.select(Todo::as_select()).load(conn))
        .await?
        .unwrap();

    Ok(Json(todos))
}

async fn create_todo(
    State(pool): State<Pool>, Json(new_todo): Json<NewTodo>) -> Result<Json<Todo>> {
    let conn = pool.get().await?;
    let new_todo = conn
        .interact(move |conn| {
            diesel::insert_into(todos::table)
                .values(&new_todo)
                .get_result(conn)
        })
        .await?
        .unwrap();

    Ok(Json(new_todo))
}

async fn delete_todo(State(pool): State<Pool>, Path(id): Path<i32>) -> Result<String> {
    let conn = pool.get().await?;
    let _ = conn
        .interact(move |conn| {
            diesel::delete(todos::table)
                .filter(todos::id.eq(id))
                .execute(conn)
        })
        .await?;

    Ok(format!("Successfully deleted todo!"))
}

async fn update_todo(State(pool): State<Pool>, Json(todo): Json<Todo>) -> Result<Json<Todo>> {
    let conn = pool.get().await?;
    let todo = conn
        .interact(move |conn| {
            diesel::update(todos::table.filter(todos::id.eq(todo.id)))
                .set((
                    todos::description.eq(todo.description),
                    todos::completed.eq(todo.completed),
                ))
                .get_result(conn)
        })
        .await?
        .unwrap();

    Ok(Json(todo))
}
