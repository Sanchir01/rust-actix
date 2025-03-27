use super::entity::Claims;
use jsonwebtoken::{
    DecodingKey, EncodingKey, Header, Validation, decode, encode, errors::Error as JWTError,
};
use std::env;
use std::error::Error;
use time::Duration;
use tower_cookies::{Cookie, cookie::SameSite};
use uuid::Uuid;

pub fn create_jwt(
    id: Uuid,
    title: String,
    slug: String,
    expiration: u64,
) -> Result<String, Box<dyn Error>> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let my_claims = Claims {
        id,
        title,
        slug,
        exp: expiration as usize,
    };
    let token = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;

    Ok(token)
}

pub fn create_cookie(token: &str, name: &str) -> Cookie<'static> {
    let mut cookie = Cookie::new(name.to_string(), token.to_string());
    cookie.set_path("/");
    cookie.set_secure(true);
    cookie.set_http_only(true);
    cookie.set_same_site(SameSite::Lax);
    cookie.set_max_age(Duration::seconds(3600));

    cookie
}

pub fn parse_token(token: &str) -> Result<Claims, JWTError> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let validation = Validation::default();
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )?;
    Ok(token_data.claims)
}
