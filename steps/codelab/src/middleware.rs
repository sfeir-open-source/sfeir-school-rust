use axum::{async_trait, extract::{FromRequestParts}, RequestPartsExt};
use axum::http::{HeaderMap, Request};
use axum::http::request::Parts;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
  pub secret_header: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
    where
        S: Send + Sync ,
{
    type Rejection = String;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
      let accept_header = parts
        .headers
        .get("fake-auth")
        .map(|value| value.to_str());
      match accept_header {
        Some(Ok("sfeir-2024")) => {
          Ok(Claims {secret_header: "success".to_string()})
        }
        _ => Err("Unauthorized".to_string())
      }
    }
}
