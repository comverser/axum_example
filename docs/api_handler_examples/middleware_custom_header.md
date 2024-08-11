```rs
use axum::routing::get;
use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};
use axum::{middleware, Extension, Router};

#[derive(Clone)]
struct HeaderMessage(pub String);

pub fn create_middleware_router() -> Router {
    Router::new()
        .route(
            "/read_middleware_custom_header",
            get(read_middleware_custom_header),
        )
        .route_layer(middleware::from_fn(set_middleware_custom_header))
}

async fn read_middleware_custom_header(Extension(message): Extension<HeaderMessage>) -> String {
    message.0
}

async fn set_middleware_custom_header(
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let headers = request.headers();
    let message_value = headers
        .get("message")
        .ok_or_else(|| StatusCode::BAD_REQUEST)?;

    let message = message_value
        .to_str()
        .map_err(|_error| StatusCode::BAD_REQUEST)?
        .to_owned();

    let extensions = request.extensions_mut();

    extensions.insert(HeaderMessage(message));

    Ok(next.run(request).await)
}
```