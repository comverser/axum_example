use axum::{extract::State, http::StatusCode, Extension, Json};
// use axum_extra::{
//     headers::{authorization::Bearer, Authorization},
//     TypedHeader,
// };
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    Set,
};
use serde::{Deserialize, Serialize};

use crate::periphery_adapters::{
    driven_infrastructure_interface::persistence_adapter::sea_orm::{
        prelude::Users,
        users::{self, Model},
    },
    driving_presentation_interface::gui_controller::utils::jwt::create_jwt,
};

#[derive(Deserialize)]
pub struct RequestUser {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct ResponseUser {
    username: String,
    id: i32,
    token: String,
}

pub async fn create_user(
    State(database_connection): State<DatabaseConnection>,
    Json(request_user): Json<RequestUser>,
) -> Result<Json<ResponseUser>, StatusCode> {
    let jwt = create_jwt()?;

    let new_user = users::ActiveModel {
        username: Set(request_user.username),
        password: Set(hash_password(request_user.password)?),
        token: Set(Some(jwt)),
        ..Default::default()
    }
    .save(&database_connection)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ResponseUser {
        username: new_user.username.unwrap(),
        id: new_user.id.unwrap(),
        token: new_user.token.unwrap().unwrap(),
    }))
}

pub async fn login(
    State(database_connection): State<DatabaseConnection>,
    Json(request_user): Json<RequestUser>,
) -> Result<Json<ResponseUser>, StatusCode> {
    let db_user = Users::find()
        .filter(users::Column::Username.eq(request_user.username))
        .one(&database_connection)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(db_user) = db_user {
        if !verify_password(request_user.password, &db_user.password)? {
            return Err(StatusCode::UNAUTHORIZED);
        }

        let new_token = create_jwt()?;
        let mut user = db_user.into_active_model();

        user.token = Set(Some(new_token));

        let saved_user = user
            .save(&database_connection)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(Json(ResponseUser {
            username: saved_user.username.unwrap(),
            id: saved_user.id.unwrap(),
            token: saved_user.token.unwrap().unwrap(),
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn logout(
    State(database_connection): State<DatabaseConnection>,
    // authorization: TypedHeader<Authorization<Bearer>>,
    Extension(user): Extension<Model>,
) -> Result<(), StatusCode> {
    // let token = authorization.token();
    // let mut user = if let Some(user) = Users::find()
    //     .filter(users::Column::Token.eq(Some(token)))
    //     .one(&database_connection)
    //     .await
    //     .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?
    // {
    //     user.into_active_model()
    // } else {
    //     return Err(StatusCode::NOT_FOUND);
    // };

    let mut user = user.into_active_model();

    user.token = Set(None);

    user.save(&database_connection)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}

fn hash_password(password: String) -> Result<String, StatusCode> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST).map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)
}

fn verify_password(password: String, hash: &str) -> Result<bool, StatusCode> {
    bcrypt::verify(password, hash).map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)
}
