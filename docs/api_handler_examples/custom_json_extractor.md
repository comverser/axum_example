```rs
use axum::{
    async_trait,
    extract::{FromRequest, Request},
    http::StatusCode,
    Json, RequestExt,
};
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct RequestUser {
    #[validate(email(message = "Invalid email address"))]
    username: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    password: String,
}

#[async_trait]
impl<S> FromRequest<S> for RequestUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: Request, _state: &S) -> Result<Self, Self::Rejection> {
        let Json(user): Json<RequestUser> = req.extract().await.map_err(|error| {
            (
                StatusCode::BAD_REQUEST,
                format!("Failed to extract JSON from request: {:?}", error),
            )
        })?;

        if let Err(errors) = user.validate() {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("Validation failed: {:?}", errors),
            ));
        }

        Ok(user)
    }
}

pub async fn custom_json_extractor(user: RequestUser) {
    dbg!(user);
}
```