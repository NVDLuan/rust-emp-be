use crate::configs::database::DbPool;
use crate::modules::authentication::model::Model as UserModel;
use crate::modules::authentication::repository::repository::UserRepository;
use actix_web::body::BoxBody;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{Error, HttpMessage, HttpResponse};
use futures::future::{LocalBoxFuture, Ready, ok};
use jsonwebtoken::{DecodingKey, Validation, decode};
use std::rc::Rc;
use std::task::{Context, Poll};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

const SECRET_KEY: &[u8] = b"your_secret_key";

#[derive(Debug, serde::Deserialize)]
struct Claims {
    sub: Uuid,  // User ID
    exp: usize, // Expiration time
}

pub struct AuthMiddleware {
    pub db: Rc<DbPool>,
}

impl<S> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Transform = AuthMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Ready<Result<Self::Transform, Self::InitError>> {
        ok(AuthMiddlewareService {
            service: Rc::new(service),
            db: self.db.clone(),
        })
    }
}

pub struct AuthMiddlewareService<S> {
    service: Rc<S>,
    db: Rc<DbPool>,
}

impl<S> Service<ServiceRequest> for AuthMiddlewareService<S>
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
            match get_user_from_request(&req, &db).await {
                Ok(user) => {
                    req.extensions_mut().insert(user);
                    service.call(req).await
                }
                Err(err) => {
                    let response = HttpResponse::Unauthorized().json(err).map_into_boxed_body(); // Ensure BoxBody
                    Ok(ServiceResponse::new(req.into_parts().0, response))
                }
            }
        })
    }
}

async fn get_user_from_request(
    req: &ServiceRequest,
    db: &DbPool,
) -> Result<UserModel, &'static str> {
    if let Some(access_cookie) = req.cookie("access_token") {
        return get_user_from_token(access_cookie.value(), db).await;
    }

    if let Some(auth) = req.headers().get("Authorization") {
        let token = auth.to_str().map_err(|_| "Invalid header format")?;
        if token.starts_with("Bearer ") {
            return get_user_from_token(&token[7..], db).await;
        }
    }

    Err("Missing token")
}

async fn get_user_from_token(token: &str, db: &DbPool) -> Result<UserModel, &'static str> {
    // Decode the token
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET_KEY),
        &Validation::default(),
    )
    .map_err(|_| "Invalid token")?;

    // Get the current time as a Unix timestamp in seconds
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| "Failed to get current time")?
        .as_secs() as usize;

    // Check if the token has expired
    if current_time >= token_data.claims.exp {
        return Err("Token has expired");
    }

    // Token is valid, proceed to fetch the user
    let user_id = token_data.claims.sub;
    UserRepository::get_user_by_id(db, user_id)
        .await
        .map_err(|_| "User not found")
}
