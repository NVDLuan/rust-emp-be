use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
mod middlewares;
mod shared;
mod system;
mod user;

pub fn handle_subscribe_message(topic: String, payload: Vec<u8>) {
    if topic.starts_with("system/") {
        system::subscriber::handle_message(topic, payload);
    } else {
        log::info!("⚙️ Unhandled topic: {}", topic);
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    user::router::init_auth_routes(cfg);
}


#[derive(OpenApi)]
#[openapi()]
pub struct BaseApiDoc;

pub fn swagger_routes(cfg: &mut web::ServiceConfig) {
    let mut api = BaseApiDoc::openapi();
    api.merge(user::openapi::AuthApiDoc::openapi());

    cfg.service(
        SwaggerUi::new("/docs/{_:.*}").url("/api-docs/openapi.json", api),
    );
}