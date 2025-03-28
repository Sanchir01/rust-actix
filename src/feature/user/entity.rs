use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use ts_rs::TS;
use utoipa::ToSchema;
use uuid::Uuid;
use validator_derive::Validate;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema, TS)]
#[ts(export)]
pub struct User {
    #[schema(value_type = String, format = Uuid)]
    id: Uuid,
    title: String,
    slug: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub exp: usize,
}

#[derive(Deserialize, Validate, Debug, TS)]
#[ts(export)]
pub struct CreateUserRequest {
    #[validate(length(min = 1))]
    pub title: String,
    #[validate(length(min = 1))]
    pub slug: String,
}
