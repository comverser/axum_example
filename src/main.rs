use axum_example::periphery_adapters::driving_presentation_interface::gui_controller::create_app;

#[tokio::main]
async fn main() {
    create_app().await;
}
