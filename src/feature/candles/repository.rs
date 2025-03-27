use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub trait CandlesRepositoryTrait {}

#[derive(Clone)]
pub struct CandlesRepository {
    candles_repo: Pool<Postgres>,
}

impl CandlesRepository {
    pub fn new_candles_repository(candles_repo: Pool<Postgres>) -> Self {
        Self { candles_repo }
    }
}

impl CandlesRepositoryTrait for CandlesRepository {}
