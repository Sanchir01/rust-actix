use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use ts_rs::TS;
use utoipa::ToSchema;
use uuid::Uuid;
#[derive(Serialize, Deserialize, FromRow, ToSchema, TS)]
#[ts(export)]
pub struct User {
    #[schema(value_type = String, format = Uuid)]
    id: Uuid,
    name: String,
    description: String,
}
