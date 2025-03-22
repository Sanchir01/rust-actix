use std::sync::Arc;

use tokio::sync::Mutex;

use super::repository::UserRepository;
use crate::feature::user::entity::User;

#[derive(Clone)]
pub struct UserService {
    user_repo: Arc<UserRepository>,
}

impl UserService {
    pub fn new_user_services(user_repo: Arc<UserRepository>) -> Self {
        Self { user_repo }
    }
}
