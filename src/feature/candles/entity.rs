use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use ts_rs::TS;
use utoipa::ToSchema;
use uuid::Uuid;
use validator_derive::Validate;
#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema, TS)]
#[ts(export)]
pub struct CandlesStruct {
    #[schema(value_type = String, format = Uuid)]
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub version: i64,
    pub price: i64,
    #[schema(value_type = String, format = Uuid)]
    pub color_id: Uuid,
}

#[derive(Debug, Deserialize, ToSchema, TS, Validate)]
#[ts(export)]
pub struct CreateCandleRequest {
    #[schema(value_type = String, format = Uuid)]
    pub color_id: Uuid,
    #[validate(length(min = 1))]
    pub title: String,
    #[validate(range(min = 1))]
    pub version: i64,
    #[validate(range(min = 1))]
    pub price: i64,
}
