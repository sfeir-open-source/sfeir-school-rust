# Instructions
## 1) Créer un projet

```Bash
# Créer un projet
cargo new battleship

# charger les dépendences
cargo add axum 

#Charger la lib pour l'asynchrone
cargo add -F  rt,rt-multi-thread,macros tokio
```
   
Et le code suivant dans le fichier `src/main.rs` :
```Rust
use axum::{Router, routing::get};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {

  let app = Router::new()
    .route("/", get(root));

  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
  axum::serve(listener, app).await.unwrap();
}


async fn root() -> &'static str {
  "Hello, World!"
}

```

## 2) Stockons notre première valeur

Il va falloir créer une nouvelle partie dans SQL. Créons un pool de connexion Postgres.

```shell
cargo add -F runtime-async-std-native-tls,postgres,macros,chrono sqlx@0.6
```

```Rust
// [...]
use sqlx::PgPool;

// [...]
#[derive(Clone)]
pub struct AppState {
  pub db_pool: PgPool,
}

async fn init_shared_state() -> AppState{
  match PgPool::connect("url").await {
    Ok(db_pool) => {
      AppState {
        db_pool
      }
    }
    Err(e) => panic!("pg pool error : {}", e)
  }
}
```

On édite la fonction main pour lier le state à notre application :

```Rust
// [...]{
  let app = Router::new()
    .route("/", get(root))
    .with_state(init_shared_state().await);
// [...]
```


Voici un exemple des imports et d'une fonction qui écrit un nombre dans une table :

```Rust
struct LabValue {
  id: i32,
  value: i64
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
```

Créer une route pour créer cette valeur dans la base de données :

```Rust
// [...]
  .route("/", get(root))
  .route("/value/:integer", post(create_number))

// [...]
use axum::extract::{Path, State};

async fn create_number(State(state): State<AppState>, Path(value): Path<i64>) ->  String {
  match create(&state.db_pool ,value).await {
    Ok(value) => value.id.to_string() + " worth " + &value.value.to_string() + "\n",
    Err(e) => e.to_string()
  }
}
```

## 3) On écrit ? On protège !

Pour créer un middleware nous allons procéder de la manière suivante. 
Créons un nouveau fichier avec une structure que nous demanderons aux routes que nous voulons protéger.
Cette structure devra implémenter le trait `FromRequestParts`

```Rust
use axum::{async_trait, extract::{FromRequestParts, TypedHeader}, RequestPartsExt};
use axum::headers::Authorization;
use axum::headers::authorization::Bearer;
use axum::http::{request::Parts};

use crate::api::app_state::AppStateTrait;
use crate::model::error::ASError;
use crate::usecases::users::claims::Claims;

#[async_trait]
impl<S> FromRequestParts<S> for Claims
    where
        S: Send + Sync + AppStateTrait,
{
    type Rejection = ASError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| ASError::InvalidToken)?;
        // Decode the user data
        let claims = Claims::extract_token(bearer.token().to_string(), state.get_secret())
            .map_err(|_| ASError::InvalidToken)?;

        Ok(claims)
    }
}
```


## 4) Créer un plateau de jeu

Pour la route `POST http://localhost:3000/play`, créez une nouvelle partie dans la table `games`.
Créez aussi 5 cases( vous pouvez utiliser la fonction generate_ships du fichier utils.rs) que vous sauvegarderez dans 
la table `cases`.

## 5) Jouons un tour !

Il est temps de créer une route `PATCH http://localhost:3000/play` ayant pour body `{ case: String, player: String }` qui retourne si le coup est un touché ou non.
Elle prendra en paramètre la position du coup.
La réponse sera du type `{finished: boolean, hit: boolean}`

aide : pour lire un body JSON
  ```Rust
use axum::extract::Json;
use serde::Deserialize;

#[derive(Deserialize)]
struct CreateUser {
  email: String,
  password: String,
}

async fn create_user(Json(payload): Json<CreateUser>) {
  // payload is a `CreateUser`
}
```

## 7) Des statistiques du joueur 

Créez la route `GET http://localhost:3000/player/stats` qui retourne les statistiques du joueur.
La réponse sera du type `[['name', 'value']]`. 
En Rust, vous pouvez utiliser JSON<Vec<Vec<String>>>.

```Rust
async fn root() -> Json<Vec<Vec<String>>> {
  Json(vec![vec!["Hello, World!".to_owned()]])
}
```


## 8) Rajouter Une réponse ? Les jurons du captain Hadook ? 

Modifier la route `POST http://localhost:3000/play` pour rajouter un champs field
