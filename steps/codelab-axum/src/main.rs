use axum::{Router, routing::{get, post}};
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

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
async fn root() -> Response {
  to_response(StatusCode::INTERNAL_SERVER_ERROR, "".to_owned())
}

fn to_response<U>( status: StatusCode, data: U) -> Response where U: IntoResponse {
  let mut res = data.into_response();
  *res.status_mut() = status;
  res
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

#[derive(Deserialize)]
struct Turn {
  case: String,
  player: String,
}


async fn create_number(State(state): State<AppState>, Path(value): Path<i64>, _claim: Claims) ->  String {
  match create(&state.db_pool ,value).await {
    Ok(value) => value.id.to_string() + " worth " + &value.value.to_string(),
    Err(e) => e.to_string()
  }
}
use serde::Deserialize;
use axum::extract::Json;

async fn play(State(state): State<AppState>, Json(payload): Json<Turn>, _claim: Claims) ->  String {
  match play_turn(&state.db_pool ,payload).await {
    Ok(value) =>  " test ".to_string(),
    Err(e) => e.to_string()
  }
}

struct TurnValue {
  pub id: i32,
  pub turn: Option<i32>,
  pub player: String
}



struct TurnDTO {
  pub id: i32,
  pub turn: i32,
  pub player: String
}

async fn play_turn(pool: &PgPool, value: Turn) -> Result<TurnValue, String>  {
  query_as!(TurnValue,
            "SELECT id, turn, player from games",
        )
    .fetch_one(pool)
    .await;
  Err("not implemented".to_owned())
}




async fn create(pool: &PgPool, value: i64) -> Result<LabValue, Error>  {
  let vec = vec![1,2,3,4,5];
  query_as!(LabValue,
            "INSERT INTO tests (val)
                SELECT * FROM  UNNEST ($1::integer[]) RETURNING id, val as value",
    json!(vec)
        )
    .fetch_one(pool)
    .await
}
