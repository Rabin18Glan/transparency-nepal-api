use crate::core::error::AppError;
use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use chrono::{Duration, Utc};
use rusty_paseto::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthUser {
    pub phone_number: String,
}

use std::sync::Arc;

#[derive(Clone)]
pub struct PasetoAuth {
    key: Arc<PasetoSymmetricKey<V4, Local>>,
}

impl PasetoAuth {
    pub fn new(secret: &str) -> Result<Self, AppError> {
        let key_bytes: [u8; 32] = secret.as_bytes().try_into().map_err(|_| {
            AppError::InternalServerError("PASETO_SECRET must be exactly 32 bytes".to_string())
        })?;

        let key = PasetoSymmetricKey::<V4, Local>::from(Key::from(key_bytes));

        Ok(Self { key: Arc::new(key) })
    }

    pub fn create_token(&self, phone_number: &str) -> Result<String, AppError> {
        let exp = (Utc::now() + Duration::days(7)).to_rfc3339();
        let expiration_claim = ExpirationClaim::try_from(exp)
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        // CustomClaim expects a tuple. The value must be something convertible to a serde_json::Value or directly a primitive?
        // rusty_paseto CustomClaim::try_from typically takes (&str, &str) or (&str, serde_json::Value).
        let custom_claim = CustomClaim::try_from(("phone_number", phone_number))
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        let token = PasetoBuilder::<V4, Local>::default()
            .set_claim(expiration_claim)
            .set_claim(custom_claim)
            .build(&*self.key)
            .map_err(|e| AppError::InternalServerError(format!("Token creation failed: {}", e)))?;

        Ok(token)
    }

    pub fn verify_token(&self, token: &str) -> Result<AuthUser, AppError> {
        let mut parser = PasetoParser::<V4, Local>::default();
        let parsed_token = parser
            .parse(token, &*self.key)
            .map_err(|_| AppError::Unauthorized)?;

        let phone_number = parsed_token
            .get("phone_number")
            .and_then(|v| v.as_str())
            .ok_or(AppError::Unauthorized)?
            .to_string();

        Ok(AuthUser { phone_number })
    }
}

#[async_trait]
impl FromRequestParts<crate::core::state::SharedState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &crate::core::state::SharedState,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|value| value.to_str().ok())
            .ok_or(AppError::Unauthorized)?;

        if !auth_header.starts_with("Bearer ") {
            return Err(AppError::Unauthorized);
        }

        let token = &auth_header[7..];

        let user = state.paseto.verify_token(token)?;

        Ok(user)
    }
}
