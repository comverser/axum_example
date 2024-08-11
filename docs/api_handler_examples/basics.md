```rs
use axum::extract::{Json, Path, Query};
use axum::http::header::HeaderMap;
use axum_extra::TypedHeader;
use headers::UserAgent;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct MirrorJson {
    message: String,
}

#[derive(Serialize)]
pub struct MirrorJsonResponse {
    message: String,
    message_from_server: String,
}

#[derive(Deserialize, Serialize)]
pub struct QueryParams {
    pub name: String,
    pub age: u8,
}

pub async fn mirror_body_string(body: String) -> String {
    body
}

pub async fn mirror_body_json(Json(payload): Json<MirrorJson>) -> Json<MirrorJsonResponse> {
    dbg!(&payload);
    Json(MirrorJsonResponse {
        message: payload.message,
        message_from_server: "Hello from server".to_string(),
    })
}

pub async fn mirror_path_variables(Path(id): Path<i32>) -> String {
    id.to_string()
}

pub async fn mirror_query_params(Query(query): Query<QueryParams>) -> Json<QueryParams> {
    Json(query)
}

pub async fn mirror_custom_header(headers: HeaderMap) -> String {
    headers
        .get("x-message")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

pub async fn mirror_user_agent(TypedHeader(user_agent): TypedHeader<UserAgent>) -> String {
    user_agent.to_string()
}
```