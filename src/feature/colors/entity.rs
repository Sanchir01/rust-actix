use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use ts_rs::TS;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow, TS)]
#[ts(export)]
pub struct ColorsStruct {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub version: i64,
}
