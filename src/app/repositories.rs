use crate::feature::{
    candles::repository::CandlesRepository, colors::repository::ColorRepository,
    user::repository::UserRepository,
};
use sqlx::{Pool, Postgres};
use std::sync::Arc;

#[derive(Clone)]
pub struct Repositories {
    pub user_repository: Arc<UserRepository>,
    pub candles_repository: Arc<CandlesRepository>,
    pub color_repository: Arc<ColorRepository>,
}

impl Repositories {
    pub fn new_repositories(db: Pool<Postgres>) -> Self {
        Self {
            user_repository: Arc::new(UserRepository::new_user_repository(db.clone())),
            candles_repository: Arc::new(CandlesRepository::new_candles_repository(db.clone())),
            color_repository: Arc::new(ColorRepository::new_color_repository(db.clone())),
        }
    }
}
