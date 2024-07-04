use axum::{Router, routing::get};
use sqlx::PgPool;
use sqlx::{query_as};
use sqlx::Error;

mod middleware;
mod utils;

use utils::generate_ships;

#[tokio::main]
async fn main() {
  let app = Router::new()
    .route("/", get(root))
    .route("/value/:integer", get(create_number))
    .with_state(init_shared_state().await);

  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
  axum::serve(listener, app).await.unwrap();
}


async fn root() -> &'static str {
  "Hello, World!"
}

#[derive(Clone)]
pub struct AppState {
  pub db_pool: PgPool,
}

async fn init_shared_state() -> AppState {
  match PgPool::connect("postgres://laborantin:SeeLa2024Jeqvb@media.yann-lesage.fr:15152").await {
    Ok(db_pool) => {
      AppState {
        db_pool
      }
    }
    Err(e) => panic!("Init pool error : {}", e)
  }
}

struct LabValue {
  pub id: i32,
  pub value: i64
}

use axum::extract::{Path, State};
use crate::middleware::Claims;

async fn create_number(State(state): State<AppState>, Path(value): Path<i64>, _claim: Claims) ->  String {
   match create(&state.db_pool ,value).await {
    Ok(value) => value.id.to_string() + " worth " + &value.value.to_string(),
    Err(e) => e.to_string()
   }
}

async fn create(pool: &PgPool, value: i64) -> Result<LabValue, Error>  {
  query_as!(LabValue,
            "INSERT INTO tests (val)
                VALUES ($1) RETURNING id, val as value",
            value
        )
    .fetch_one(pool)
    .await
}
