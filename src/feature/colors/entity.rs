use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use ts_rs::TS;
use uuid::Uuid;

use validator_derive::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow, TS)]
#[ts(export)]
pub struct ColorsStruct {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub version: i64,
}
#[derive(Debug, Validate, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub struct CreateColorRequest {
    #[validate(length(min = 1, message = "Title cannot be empty"))]
    pub title: String,

    #[validate(range(min = 1, message = "version must be at least 1"))]
    pub version: i64,
}
