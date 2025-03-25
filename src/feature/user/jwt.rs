use std::error::Error;
use uuid::Uuid;
use jsonwebtoken::{encode, Header, EncodingKey};
use std::env;
use super::entity::Claims;

pub fn create_jwt(id: Uuid, title: String, slug: String) -> Result<String, Box<dyn Error>> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let my_claims = Claims {
        id,
        title,
        slug,
    };
    let token = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(secret.as_bytes())
    )?;
    
    Ok(token)
}