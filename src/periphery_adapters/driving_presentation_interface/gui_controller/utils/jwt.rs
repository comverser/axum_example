use axum::http::StatusCode;
use chrono::{Duration, Utc};
use dotenvy_macro::dotenv;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::periphery_adapters::driving_presentation_interface::gui_controller::Config;

use super::app_error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: usize,
    iat: usize,
}

pub fn create_jwt() -> Result<String, StatusCode> {
    let mut now = Utc::now();
    let iat = now.timestamp() as usize;
    // By default, this window is 60 seconds (30 seconds before and 30 seconds after the expiration time).
    // This approach accounts for potential clock skew between the client and server.
    let expires_in = Duration::seconds(30);
    now += expires_in;
    let exp = now.timestamp() as usize;
    let claim = Claims { exp, iat };
    let key = EncodingKey::from_secret(dotenv!("JWT_SECRET").as_bytes());

    encode(&Header::default(), &claim, &key).map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn is_valid(jwt: &str) -> Result<bool, AppError> {
    let jwt_secret = Config::from_env().jwt_secret;
    let decoding_key = DecodingKey::from_secret(jwt_secret.as_bytes());

    decode::<Claims>(
        jwt,
        &decoding_key,
        &Validation::new(jsonwebtoken::Algorithm::HS256),
    )
    .map_err(|error| match error.kind() {
        jsonwebtoken::errors::ErrorKind::ExpiredSignature => AppError::new(
            StatusCode::UNAUTHORIZED,
            "Your session has expired, please login again",
        ), // todo: address the expired token scenario
        _ => AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong, please try again",
        ),
    })?;

    Ok(true)
}
