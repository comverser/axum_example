mod handlers;

use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use super::middleware_setup::{authenticate_with_bearer_token, AppState};

use handlers::{
    atomic_update_task::atomic_update_task,
    create_task::create_task,
    delete_task::delete_task,
    get_task::{get_all_tasks, get_one_task},
    partial_update_task::partial_update_task,
    reading_list::{
        add_book, delete_book, edit_book, get_book, get_books, index, search_books, update_book,
    },
    shared_mutable_state::shared_mutable_state,
    users::{create_user, login, logout},
};

pub fn create_router(app_state: AppState) -> Router<AppState> {
    Router::new()
        .route("/state", get(shared_mutable_state))
        .nest("/users", create_users_router(app_state))
        .nest("/tasks", create_tasks_router())
        .nest("/reading_list", create_reading_list_router())
}

fn create_reading_list_router() -> Router<AppState> {
    Router::new()
        .route("/", get(index))
        .route("/books", get(get_books).post(add_book))
        .route(
            "/books/:book_id",
            get(get_book).put(update_book).delete(delete_book),
        )
        .route("/books/edit/:book_id", get(edit_book))
        .route("/books/search", post(search_books))
}

fn create_users_router(app_state: AppState) -> Router<AppState> {
    Router::new()
        .route("/logout", post(logout))
        .route_layer(middleware::from_fn_with_state(
            app_state,
            authenticate_with_bearer_token,
        ))
        .route("/", post(create_user))
        .route("/login", post(login))
}

fn create_tasks_router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_all_tasks).post(create_task))
        .route(
            "/:task_id",
            get(get_one_task)
                .put(atomic_update_task)
                .patch(partial_update_task)
                .delete(delete_task),
        )
}
