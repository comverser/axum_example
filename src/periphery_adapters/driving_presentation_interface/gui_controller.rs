mod api_routing;
mod middleware_setup;
mod utils;

use dotenvy_macro::dotenv;
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;

use {api_routing::create_router, middleware_setup::setup_app_state};

struct Config<'a> {
    port: u16,
    database_url: &'a str,
    jwt_secret: &'a str,
    production: bool,
    gui_view_workspace_path: &'a str,
}

impl<'a> Config<'a> {
    pub fn from_env() -> Self {
        let database_url = dotenv!("DATABASE_URL");
        let jwt_secret = dotenv!("JWT_SECRET");
        let port = dotenv!("PORT").parse().unwrap();
        let production = dotenv!("PRODUCTION").parse().unwrap();
        let gui_view_workspace_path = dotenv!("GUI_VIEW_WORKSPACE_PATH");

        Config {
            database_url,
            jwt_secret,
            port,
            production,
            gui_view_workspace_path,
        }
    }
}

pub async fn create_app() {
    let config = Config::from_env();
    // todo: investigate why it still works. Isn't the configuration content is moved?
    // let test = config.database_url;

    let app_state = setup_app_state(config.database_url).await;
    let router = create_router(app_state.clone());
    let cors_setup = middleware_setup::setup_cors().await;

    let view_path = config.gui_view_workspace_path;
    let serve_dir = ServeDir::new(format!("{view_path}/static"))
        .not_found_service(ServeDir::new(format!("{view_path}/templates/error.html")));

    let mut app = router
        .nest_service("/static", serve_dir)
        .with_state(app_state)
        .layer(cors_setup);

    if config.production == false {
        app = app.layer(LiveReloadLayer::new());
    }

    let address = format!("0.0.0.0:{}", config.port);
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    if config.production {
        println!("🚀 Server started successfully in production mode");
    } else {
        println!("🚀 Server started successfully in debug mode");
    }

    axum::serve(listener, app).await.unwrap();
}
