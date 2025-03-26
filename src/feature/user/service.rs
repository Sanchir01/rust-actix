use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use tower_cookies::Cookie;
use uuid::Uuid;

use super::jwt::create_jwt;
use super::repository::UserRepositoryTrait;
use crate::feature::user::entity::User;
use crate::feature::user::jwt::create_cookie;

#[derive(Clone)]
pub struct UserService<T: UserRepositoryTrait + Send + Sync + 'static> {
    user_repo: Arc<T>,
}

impl<T: UserRepositoryTrait + Send + Sync + 'static> UserService<T> {
    pub fn new_user_services(user_repo: Arc<T>) -> Self {
        Self { user_repo }
    }

    pub async fn get_users(&self) -> Result<Vec<User>, sqlx::Error> {
        self.user_repo.get_all_users().await
    }

    pub async fn create_user(
        &self,
        title: &str,
        slug: &str,
    ) -> Result<(Uuid, Cookie<'static>), Box<dyn std::error::Error>> {
        let user_id = self.user_repo.create_user(title, slug).await?;
        let expiration = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() + 3600;
        let jwt = create_jwt(user_id, title.to_string(), slug.to_string(), expiration)?;
        println!("Generated JWT token: {}", jwt);
        let cookies = create_cookie(&jwt, "refresh");
        Ok((user_id, cookies))
    }
}
