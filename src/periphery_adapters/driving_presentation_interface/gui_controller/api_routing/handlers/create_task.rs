use axum::{extract::State, http::StatusCode, Json};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::Deserialize;

use crate::periphery_adapters::driven_infrastructure_interface::persistence_adapter::sea_orm::{
    prelude::Users, tasks, users,
};

#[derive(Deserialize)]
pub struct RequestTask {
    priority: Option<String>,
    title: String,
    description: Option<String>,
}

pub async fn create_task(
    State(database_connection): State<DatabaseConnection>,
    authorization: TypedHeader<Authorization<Bearer>>,
    Json(request_task): Json<RequestTask>,
) -> Result<(), StatusCode> {
    let token = authorization.token();

    let users = if let Some(user) = Users::find()
        .filter(users::Column::Token.eq(Some(token)))
        .one(&database_connection)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        user
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let new_task = tasks::ActiveModel {
        priority: Set(request_task.priority),
        title: Set(request_task.title),
        description: Set(request_task.description),
        user_id: Set(Some(users.id)),
        ..Default::default()
    };
    dbg!(&new_task);

    let _result = new_task.save(&database_connection).await.unwrap();
    dbg!(_result);

    Ok(())
}
