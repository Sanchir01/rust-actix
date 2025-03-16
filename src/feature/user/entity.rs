use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Serialize, Deserialize,FromRow)]
pub struct User {
    id: Uuid,
    name: String,
    description: String,
}