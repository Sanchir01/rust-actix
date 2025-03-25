use std::sync::Arc;

use uuid::Uuid;

use super::repository::UserRepository;
use crate::feature::user::entity::User;
use super::jwt::create_jwt;

#[derive(Clone)]
pub struct UserService {
    user_repo: Arc<UserRepository>,
}

impl UserService {
    pub fn new_user_services(user_repo: Arc<UserRepository>) -> Self {
        Self { user_repo }
    }
    pub async fn get_users(&self) -> Result<Vec<User>, sqlx::Error> {
        self.user_repo.get_all_users().await
    }
    pub async fn create_user(&self, title: &str, slug: &str) -> Result<Uuid, Box<dyn std::error::Error>> {
        let user_id = self.user_repo.create_user(title, slug).await?;
        let jwt = create_jwt(user_id, title.to_string(), slug.to_string())?;
        println!("Generated JWT token: {}", jwt);
        Ok(user_id)
    }
}
