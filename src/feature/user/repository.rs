use crate::{feature::user::entity::User, utils::errors_message::ErrorMessage};
#[cfg(test)]
use mockall::{automock, predicate::*};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[cfg_attr(test, automock)]
pub trait UserRepositoryTrait {
    async fn get_all_users(&self) -> Result<Vec<User>, sqlx::Error>;
    async fn create_user(
        &self,
        title: &str,
        slug: &str,
        email: &str,
        phone: &str,
        password: &str,
    ) -> Result<Uuid, sqlx::Error>;
    async fn get_user_by_phone(&self, phone: &str) -> Result<User, ErrorMessage>;
    async fn get_user_by_id(&self, id: Uuid) -> Result<User, ErrorMessage>;
}
#[derive(Clone)]
pub struct UserRepository {
    user_repo: Pool<Postgres>,
}
impl UserRepository {
    pub fn new_user_repository(user_repo: Pool<Postgres>) -> Self {
        Self { user_repo }
    }
}

impl UserRepositoryTrait for UserRepository {
    async fn get_all_users(&self) -> Result<Vec<User>, sqlx::Error> {
        let query = r#"
            SELECT id, title, email,phone,password,phone,slug,version
            FROM users
        "#;
        let users = sqlx::query_as(query)
            .fetch_all(&self.user_repo)
            .await
            .map_err(|e| {
                eprintln!("Error fetching users: {:?}", e);
                e
            })?;
        Ok(users)
    }
    async fn create_user(
        &self,
        title: &str,
        slug: &str,
        email: &str,
        phone: &str,
        password: &str,
    ) -> Result<Uuid, sqlx::Error> {
        let query = r#"
            INSERT  INTO users (title, slug,email,phone,password)
            VALUES ($1, $2, $3, $4, $5) RETURNING id
            "#;
        let user: Uuid = sqlx::query_scalar(query)
            .bind(title)
            .bind(slug)
            .bind(email)
            .bind(phone)
            .bind(password)
            .fetch_one(&self.user_repo)
            .await
            .map_err(|e| {
                eprintln!("Error fetching users: {:?}", e);
                e
            })?;
        Ok(user)
    }
    async fn get_user_by_phone(&self, phone: &str) -> Result<User, ErrorMessage> {
        let query = r#"
            SELECT id, title, email,phone,password,phone,slug,version
            FROM users WHERE phone = $1
        "#;
        let user = sqlx::query_as(query)
            .bind(phone)
            .fetch_one(&self.user_repo)
            .await
            .map_err(|e| {
                if matches!(e, sqlx::Error::RowNotFound) {
                    return ErrorMessage::NotFoundUserId;
                }
                ErrorMessage::EmailExist
            })?;
        Ok(user)
    }
    async fn get_user_by_id(&self, id: Uuid) -> Result<User, ErrorMessage> {
        let query = r#"
            SELECT id, title, email,phone,password,phone,slug,version
            FROM users WHERE id = $1
        "#;

        let user = sqlx::query_as(query)
            .bind(id)
            .fetch_one(&self.user_repo)
            .await
            .map_err(|e| {
                if matches!(e, sqlx::Error::RowNotFound) {
                    ErrorMessage::NotFoundUserId
                } else {
                    ErrorMessage::TokenNotProvided
                }
            })?;
        Ok(user)
    }
}
