use std::sync::{Arc, Mutex};

use axum::{
    extract::{FromRef, Request, State},
    http::{Method, StatusCode},
    middleware::Next,
    response::Response,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use tower::{
    layer::util::{Identity, Stack},
    ServiceBuilder,
};
use tower_http::cors::{Any, CorsLayer};

use crate::{
    application_core::domain_layer::models::{article::Article, book::Book},
    periphery_adapters::{
        driven_infrastructure_interface::persistence_adapter::{
            connect_database,
            sea_orm::{prelude::Users, users},
        },
        driving_presentation_interface::gui_controller::utils::{
            app_error::AppError, jwt::is_valid,
        },
    },
};

#[derive(Clone, FromRef)]
pub struct AppState {
    pub message: Arc<Mutex<String>>,
    pub database_connection: DatabaseConnection,
    pub my_reading_list: Arc<Mutex<Vec<Book>>>,
    pub spa_articles: Arc<Mutex<Vec<Article>>>,
}

pub async fn setup_app_state(database_url: &str) -> AppState {
    AppState {
        message: Arc::new(Mutex::new("foo".to_owned())),
        database_connection: connect_database(database_url).await.unwrap(),
        my_reading_list: Arc::new(Mutex::new(vec![
            Book {
                id: String::from("1"),
                title: String::from("The Catcher in the Rye"),
                author: String::from("J.D. Salinger"),
            },
            Book {
                id: String::from("2"),
                title: String::from("1984"),
                author: String::from("George Orwell"),
            },
        ])),
        spa_articles: Arc::new(Mutex::new(vec![
            Article {
                id: 1,
                name: String::from("Welcome to the dojo"),
                body: String::from(
                    "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempordunt \
                    ut labore et dolore magna aliqua.",
                ),
            },
            Article {
                id: 2,
                name: String::from("How to be a Ninja"),
                body: String::from("ThThis is the body of article 2This is the body of article 2This\
                 is the body of article 2This is the body of article 2This is the body of article 2\
                 This is the body of article 2This is the body of article 2This is the body of article 2\
                 This is the body of article 2This is the body of article 2is is the body of article 2"),
            },
            Article {
                id: 3,
                name: String::from("Ninja Web Dev Tips"),
                body: String::from("This is the body of article 2 testsef jdlsfj lksadjflasdj lj as\
                dsjflkdsjafljdslafjlke fsdja lfjsdlafj ksldafj ldsakjf lsdakjfl"),
            },
        ])),
    }
}

pub async fn setup_cors() -> ServiceBuilder<Stack<CorsLayer, Identity>> {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    ServiceBuilder::new().layer(cors)
}

pub async fn authenticate_with_bearer_token(
    State(database_connection): State<DatabaseConnection>,
    TypedHeader(bearer_token): TypedHeader<Authorization<Bearer>>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let jwt = bearer_token.token().to_owned();

    let user = Users::find()
        .filter(users::Column::Token.eq(Some(jwt.clone())))
        .one(&database_connection)
        .await
        .map_err(|_error| {
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
        })?;

    // Validating token after getting from the database to obsfucate that the token is wrong.
    // Feel free to move up if you are not worried about the obsfucation.
    is_valid(&jwt)?;

    let Some(user) = user else {
        return Err(AppError::new(
            StatusCode::UNAUTHORIZED,
            "Your are not authorized, please log in or create account",
        ));
    };

    request.extensions_mut().insert(user);

    Ok(next.run(request).await)
}
