use crate::infra::database::postgres::DbPool;
use crate::modules::user::schemas::schemas::{
    ApiResponse, AdminCreateUserRequest, ChangePasswordRequest, ErrorResponse, ForgotPasswordRequest, LoginRequest,
    RefreshTokenRequest, RegisterUserRequest, ResetPasswordRequest, TokenResponse,
    UpdateProfileRequest, UserResponse,
};
use crate::modules::user::service::service::AuthService;
use crate::errors::AppError;
use actix_web::cookie::Cookie;
use actix_web::{HttpRequest, HttpResponse, web};
use tracing::info;

// ===== USER REGISTRATION =====
#[utoipa::path(
    post,
    path = "/auth/register",
    tag = "Authentication",
    request_body = RegisterUserRequest,
    responses(
        (status = 201, description = "User registered successfully", body = ApiResponse<UserResponse>),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 409, description = "User already exists", body = ErrorResponse)
    )
)]
pub async fn register_user(
    pool: web::Data<DbPool>,
    user: web::Json<RegisterUserRequest>,
) -> Result<HttpResponse, AppError> {
    let new_user = user.into_inner();

    match AuthService::register_user(pool.get_ref(), &new_user).await {
        Ok(user_response) => Ok(HttpResponse::Created().json(ApiResponse {
            success: true,
            message: "User registered successfully".to_string(),
            data: Some(user_response),
        })),
        Err(err) => Err(err),
    }
}

// ===== ADMIN CREATE USER =====
#[utoipa::path(
    post,
    path = "/auth/admin/create-user",
    tag = "Authentication",
    request_body = AdminCreateUserRequest,
    responses(
        (status = 201, description = "User created successfully by admin", body = ApiResponse<UserResponse>),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 403, description = "Admin privileges required", body = ErrorResponse)
    )
)]
pub async fn admin_create_user(
    pool: web::Data<DbPool>,
    user: web::Json<AdminCreateUserRequest>,
) -> Result<HttpResponse, AppError> {
    let new_user = user.into_inner();

    match AuthService::admin_create_user(pool.get_ref(), &new_user).await {
        Ok(user_response) => Ok(HttpResponse::Created().json(ApiResponse {
            success: true,
            message: "User created successfully by admin".to_string(),
            data: Some(user_response),
        })),
        Err(err) => Err(err),
    }
}

// ===== USER LOGIN =====
#[utoipa::path(
    post,
    path = "/auth/login",
    tag = "Authentication",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = ApiResponse<UserResponse>),
        (status = 401, description = "Invalid credentials", body = ErrorResponse)
    )
)]
pub async fn login(pool: web::Data<DbPool>, req: web::Json<LoginRequest>) -> Result<HttpResponse, AppError> {
    info!("Handler: call login");

    match AuthService::login(pool.get_ref(), &req).await {
        Ok((token_response, user_response)) => {
            info!("Login successful, setting cookies");

            let access_cookie = Cookie::build("access_token", token_response.access_token)
                .path("/")
                .http_only(true)
                .secure(true)
                .max_age(actix_web::cookie::time::Duration::seconds(900)) // 15 minutes
                .finish();

            let refresh_cookie = Cookie::build("refresh_token", token_response.refresh_token)
                .path("/")
                .http_only(true)
                .secure(true)
                .max_age(actix_web::cookie::time::Duration::seconds(604800)) // 7 days
                .finish();

            Ok(HttpResponse::Ok()
                .cookie(access_cookie)
                .cookie(refresh_cookie)
                .json(ApiResponse {
                    success: true,
                    message: "Login successful".to_string(),
                    data: Some(user_response),
                }))
        }
        Err(err) => Err(err),
    }
}

// ===== REFRESH TOKEN =====
#[utoipa::path(
    post,
    path = "/auth/refresh",
    tag = "Authentication",
    request_body = RefreshTokenRequest,
    responses(
        (status = 200, description = "Token refreshed successfully", body = ApiResponse<TokenResponse>),
        (status = 401, description = "Invalid refresh token", body = ErrorResponse)
    )
)]
pub async fn refresh_token(
    pool: web::Data<DbPool>,
    req: web::Json<RefreshTokenRequest>,
) -> Result<HttpResponse, AppError> {
    match AuthService::refresh_token(pool.get_ref(), &req).await {
        Ok(token_response) => {
            let access_cookie = Cookie::build("access_token", token_response.access_token.clone())
                .path("/")
                .http_only(true)
                .secure(true)
                .max_age(actix_web::cookie::time::Duration::seconds(900))
                .finish();

            Ok(HttpResponse::Ok().cookie(access_cookie).json(ApiResponse {
                success: true,
                message: "Token refreshed successfully".to_string(),
                data: Some(token_response),
            }))
        }
        Err(err) => Err(err),
    }
}

// ===== LOGOUT =====
#[utoipa::path(
    post,
    path = "/auth/logout",
    tag = "Authentication",
    responses(
        (status = 200, description = "Logout successful", body = ApiResponse<String>),
        (status = 401, description = "Unauthorized", body = ErrorResponse)
    )
)]
pub async fn logout(_pool: web::Data<DbPool>, _req: HttpRequest) -> HttpResponse {
    // In a real implementation, you would extract user ID from JWT token
    // For now, we'll just clear the cookies
    let access_cookie = Cookie::build("access_token", "")
        .path("/")
        .http_only(true)
        .secure(true)
        .max_age(actix_web::cookie::time::Duration::seconds(0))
        .finish();

    let refresh_cookie = Cookie::build("refresh_token", "")
        .path("/")
        .http_only(true)
        .secure(true)
        .max_age(actix_web::cookie::time::Duration::seconds(0))
        .finish();

    HttpResponse::Ok()
        .cookie(access_cookie)
        .cookie(refresh_cookie)
        .json(ApiResponse {
            success: true,
            message: "Logout successful".to_string(),
            data: Some("User logged out successfully".to_string()),
        })
}

// ===== GET ALL USERS =====
#[utoipa::path(
    get,
    path = "/auth/users",
    tag = "Authentication",
    responses(
        (status = 200, description = "List of users retrieved", body = ApiResponse<Vec<UserResponse>>),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn get_all_users(pool: web::Data<DbPool>) -> HttpResponse {
    match AuthService::get_all_users(pool.get_ref()).await {
        Ok(users) => HttpResponse::Ok().json(ApiResponse {
            success: true,
            message: "Users retrieved successfully".to_string(),
            data: Some(users),
        }),
        Err(_) => HttpResponse::InternalServerError().json(ErrorResponse {
            success: false,
            message: "Failed to retrieve users".to_string(),
            error_code: Some("FETCH_USERS_FAILED".to_string()),
            details: None,
        }),
    }
}

// ===== GET USER PROFILE =====
#[utoipa::path(
    get,
    path = "/auth/profile",
    tag = "Authentication",
    responses(
        (status = 200, description = "User profile retrieved", body = ApiResponse<UserResponse>),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "User not found", body = ErrorResponse)
    )
)]
pub async fn get_profile(_pool: web::Data<DbPool>, _req: HttpRequest) -> HttpResponse {
    // In a real implementation, you would extract user ID from JWT token
    // For now, we'll return an error
    HttpResponse::Unauthorized().json(ErrorResponse {
        success: false,
        message: "Authentication required".to_string(),
        error_code: Some("UNAUTHORIZED".to_string()),
        details: None,
    })
}

// ===== UPDATE USER PROFILE =====
#[utoipa::path(
    put,
    path = "/auth/profile",
    tag = "Authentication",
    request_body = UpdateProfileRequest,
    responses(
        (status = 200, description = "Profile updated successfully", body = ApiResponse<UserResponse>),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse)
    )
)]
pub async fn update_profile(
    _pool: web::Data<DbPool>,
    _req: HttpRequest,
    _update_data: web::Json<UpdateProfileRequest>,
) -> HttpResponse {
    // In a real implementation, you would extract user ID from JWT token
    HttpResponse::Unauthorized().json(ErrorResponse {
        success: false,
        message: "Authentication required".to_string(),
        error_code: Some("UNAUTHORIZED".to_string()),
        details: None,
    })
}

// ===== CHANGE PASSWORD =====
#[utoipa::path(
    post,
    path = "/auth/change-password",
    tag = "Authentication",
    request_body = ChangePasswordRequest,
    responses(
        (status = 200, description = "Password changed successfully", body = ApiResponse<String>),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse)
    )
)]
pub async fn change_password(
    _pool: web::Data<DbPool>,
    _req: HttpRequest,
    _change_data: web::Json<ChangePasswordRequest>,
) -> HttpResponse {
    // In a real implementation, you would extract user ID from JWT token
    HttpResponse::Unauthorized().json(ErrorResponse {
        success: false,
        message: "Authentication required".to_string(),
        error_code: Some("UNAUTHORIZED".to_string()),
        details: None,
    })
}

// ===== FORGOT PASSWORD =====
#[utoipa::path(
    post,
    path = "/auth/forgot-password",
    tag = "Authentication",
    request_body = ForgotPasswordRequest,
    responses(
        (status = 200, description = "Password reset email sent", body = ApiResponse<String>),
        (status = 400, description = "Bad request", body = ErrorResponse)
    )
)]
pub async fn forgot_password(
    pool: web::Data<DbPool>,
    req: web::Json<ForgotPasswordRequest>,
) -> HttpResponse {
    match AuthService::forgot_password(pool.get_ref(), &req).await {
        Ok(_) => HttpResponse::Ok().json(ApiResponse {
            success: true,
            message: "Password reset email sent".to_string(),
            data: Some("Check your email for reset instructions".to_string()),
        }),
        Err(_) => HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            message: "Failed to send reset email".to_string(),
            error_code: Some("FORGOT_PASSWORD_FAILED".to_string()),
            details: None,
        }),
    }
}

// ===== RESET PASSWORD =====
#[utoipa::path(
    post,
    path = "/auth/reset-password",
    tag = "Authentication",
    request_body = ResetPasswordRequest,
    responses(
        (status = 200, description = "Password reset successfully", body = ApiResponse<String>),
        (status = 400, description = "Bad request", body = ErrorResponse)
    )
)]
pub async fn reset_password(
    pool: web::Data<DbPool>,
    req: web::Json<ResetPasswordRequest>,
) -> HttpResponse {
    match AuthService::reset_password(pool.get_ref(), &req).await {
        Ok(_) => HttpResponse::Ok().json(ApiResponse {
            success: true,
            message: "Password reset successfully".to_string(),
            data: Some("You can now login with your new password".to_string()),
        }),
        Err(_) => HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            message: "Failed to reset password".to_string(),
            error_code: Some("RESET_PASSWORD_FAILED".to_string()),
            details: None,
        }),
    }
}
