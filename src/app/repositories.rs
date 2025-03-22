use crate::feature::user::repository::UserRepository;
use sqlx::{Pool, Postgres};
use std::sync::Arc;

#[derive(Clone)]
pub struct Repositories {
    pub user_repository: Arc<UserRepository>,
}

impl Repositories {
    pub fn new_repositories(db: Pool<Postgres>) -> Self {
        Self {
            user_repository: Arc::new(UserRepository::new_user_repository(db)),
        }
    }
}
