use crate::infra::database::postgres::DbPool;
use crate::modules::user::model::Model as UserModel;
use crate::modules::user::repository::repository::UserRepository;
use crate::modules::user::roles::UserRole;
use crate::config::security::SecurityConfig;
use crate::errors::AppError;
use actix_web::body::BoxBody;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{Error, HttpMessage, HttpResponse};
use futures::future::{LocalBoxFuture, Ready, ok};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use std::task::{Context, Poll};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Claims {
    sub: String,  // User email
    exp: usize,   // Expiration time
    iat: usize,   // Issued at
}

pub struct AdminMiddleware {
    pub db: Rc<DbPool>,
}

impl<S> Transform<S, ServiceRequest> for AdminMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Transform = AdminMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Ready<Result<Self::Transform, Self::InitError>> {
        ok(AdminMiddlewareService {
            service: Rc::new(service),
            db: self.db.clone(),
        })
    }
}

pub struct AdminMiddlewareService<S> {
    service: Rc<S>,
    db: Rc<DbPool>,
}

impl<S> Service<ServiceRequest> for AdminMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let db = self.db.clone();
        let service = self.service.clone();

        Box::pin(async move {
            match get_admin_user_from_request(&req, &db).await {
                Ok(user) => {
                    req.extensions_mut().insert(user);
                    service.call(req).await
                }
                Err(err) => {
                    let response = HttpResponse::Forbidden().json(serde_json::json!({
                        "success": false,
                        "message": err.to_string(),
                        "error_code": "ADMIN_REQUIRED",
                        "details": None::<String>
                    })).map_into_boxed_body();
                    Ok(ServiceResponse::new(req.into_parts().0, response))
                }
            }
        })
    }
}

async fn get_admin_user_from_request(
    req: &ServiceRequest,
    db: &DbPool,
) -> Result<UserModel, AppError> {
    let token = extract_token_from_request(req)?;
    let user = get_user_from_token(&token, db).await?;

    let user_role = UserRole::from_str(&user.role)
        .ok_or_else(|| AppError::Forbidden("Invalid user role".to_string()))?;
    
    if !user_role.is_admin() {
        return Err(AppError::Forbidden("Admin privileges required".to_string()));
    }
    
    Ok(user)
}

fn extract_token_from_request(req: &ServiceRequest) -> Result<String, AppError> {
    // Try cookie first
    if let Some(cookie) = req.cookie("access_token") {
        return Ok(cookie.value().to_string());
    }
    
    // Try Authorization header
    if let Some(auth_header) = req.headers().get("Authorization") {
        let auth_str = auth_header.to_str()
            .map_err(|_| AppError::Authentication("Invalid authorization header format".to_string()))?;
        if auth_str.starts_with("Bearer ") {
            return Ok(auth_str[7..].to_string());
        }
    }
    
    Err(AppError::Authentication("Missing authentication token".to_string()))
}

async fn get_user_from_token(token: &str, db: &DbPool) -> Result<UserModel, AppError> {
    let config = SecurityConfig::load();
    
    // Decode the token
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
        &Validation::default(),
    ).map_err(|_| AppError::Authentication("Invalid token".to_string()))?;

    // Get the current time as a Unix timestamp in seconds
    let current_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|_| AppError::Internal("Failed to get current time".to_string()))?
        .as_secs() as usize;

    // Check if the token has expired
    if current_time >= token_data.claims.exp {
        return Err(AppError::Authentication("Token has expired".to_string()));
    }

    // Token is valid, proceed to fetch the user by email
    let user_email = token_data.claims.sub;
    let user = UserRepository::get_user_by_email(db, &user_email)
        .await
        .map_err(|_| AppError::NotFound("User not found".to_string()))?;
    
    // Check if user is active
    if !user.is_active {
        return Err(AppError::Forbidden("User account is deactivated".to_string()));
    }
    
    Ok(user)
}
