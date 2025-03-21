use std::sync::Arc;

use tokio::sync::Mutex;

use super::repository::UserRepository;
use crate::feature::user::entity::User;

#[derive(Clone)]
pub struct UserService {
    user_repo: Arc<Mutex<UserRepository>>,
}

impl UserService {
    pub fn new(user_repo: Arc<Mutex<UserRepository>>) -> Self {
        Self { user_repo }
    }
}
