use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use ts_rs::TS;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema, TS)]
#[ts(export)]
pub struct CandlesStruct {
    #[schema(value_type = String, format = Uuid)]
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub version: i64,
    pub price: i64,
    pub color_id: Uuid,
}
