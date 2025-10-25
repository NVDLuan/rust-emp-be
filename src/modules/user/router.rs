use actix_web::web;
use crate::modules::user::handler::auth;

pub fn init_auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/auth")
        // Public endpoints (no authentication required)
        .route("/login", web::post().to(auth::login))
        .route("/refresh", web::post().to(auth::refresh_token))
        .route("/forgot-password", web::post().to(auth::forgot_password))
        .route("/reset-password", web::post().to(auth::reset_password))
        
        // Protected endpoints (authentication required)
        .route("/logout", web::post().to(auth::logout))
        .route("/profile", web::get().to(auth::get_profile))
        .route("/profile", web::put().to(auth::update_profile))
        .route("/change-password", web::post().to(auth::change_password))
        
        // Admin-only endpoints (will be protected by middleware in app.rs)
        .route("/admin/create-user", web::post().to(auth::admin_create_user))
        .route("/admin/users", web::get().to(auth::get_all_users))
    );
}
