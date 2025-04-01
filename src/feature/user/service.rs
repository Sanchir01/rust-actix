use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use super::jwt::create_jwt;
use super::repository::{UserRepository, UserRepositoryTrait};
use crate::feature::user::entity::User;
use crate::feature::user::jwt::create_cookie;
use mockall::{automock, predicate::*};
use tower_cookies::Cookie;
use uuid::Uuid;

#[cfg_attr(test, automock)]
pub trait UserServiceTrait {
    async fn get_users(&self) -> Result<Vec<User>, sqlx::Error>;
    async fn create_user_service(
        &self,
        title: &str,
        slug: &str,
        email: &str,
        phone: &str,
    ) -> Result<(Uuid, Cookie<'static>, Cookie<'static>), Box<dyn std::error::Error>>;
    async fn get_user_by_id_service(&self, id: Uuid) -> Result<User, sqlx::Error>;
}

#[derive(Clone)]
pub struct UserService {
    user_repo: Arc<UserRepository>,
}

impl UserService {
    pub fn new_user_services(user_repo: Arc<UserRepository>) -> Self {
        Self { user_repo }
    }
}
impl UserServiceTrait for UserService {
    async fn get_users(&self) -> Result<Vec<User>, sqlx::Error> {
        self.user_repo.get_all_users().await
    }
    async fn create_user_service(
        &self,
        title: &str,
        slug: &str,
        email: &str,
        phone: &str,
    ) -> Result<(Uuid, Cookie<'static>, Cookie<'static>), Box<dyn std::error::Error>> {
        let user_id = self
            .user_repo
            .create_user(title, slug, email, phone)
            .await?;
        let expiration = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() + 3600;
        let jwt = create_jwt(user_id, title.to_string(), slug.to_string(), expiration)?;
        let refresh_token = create_cookie(&jwt, "refreshToken", 3600 * 14 * 24);
        let access_token = create_cookie(&jwt, "accessToken", 3600);
        Ok((user_id, refresh_token, access_token))
    }
    async fn get_user_by_id_service(&self, id: Uuid) -> Result<User, sqlx::Error> {
        self.user_repo.get_user_by_id(id).await
    }
}
