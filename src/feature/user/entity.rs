use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use ts_rs::TS;
use utoipa::ToSchema;
use uuid::Uuid;
#[derive(Debug,Serialize, Deserialize, FromRow, ToSchema, TS)]
#[ts(export)]
pub struct User {
    #[schema(value_type = String, format = Uuid)]
    id: Uuid,
    title: String,
    slug: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
   pub id: Uuid,
   pub title: String,
    pub slug: String,
}