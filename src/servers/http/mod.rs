use async_trait::async_trait;
use axum::{
    extract::{FromRequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    RequestPartsExt,
};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use std::env;

use crate::feature::user::entity::Claims;

pub mod server;
pub mod middleware;

#[derive(Debug)]
pub struct AuthError(String);


impl IntoResponse for AuthError{
    fn into_response(self) -> Response {
          (StatusCode::UNAUTHORIZED, self.0).into_response()
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError("Invalid or missing authorization header".to_string()))?;
        let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

       
        let token_data = decode::<Claims>(
            bearer.token(),
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::new(Algorithm::HS256),
        )
        .map_err(|_| AuthError("Invalid token".to_string()))?;

        Ok(token_data.claims)
    }
} 