use utoipa::OpenApi;
use crate::modules::user::handler::auth::*;
use crate::modules::user::schemas::schemas::*;

#[derive(OpenApi)]
#[openapi(
    paths(
        register_user,
        login,
        refresh_token,
        forgot_password,
        reset_password,
        change_password,
        update_profile
    ),
    components(schemas(
        NewUser,
        LoginRequest,
        LoginResponse,
        RefreshTokenRequest,
        TokenResponse,
        ForgotPasswordRequest,
        ResetPasswordRequest,
        ChangePasswordRequest,
        UpdateProfileRequest,
        UserResponse
    )),
    tags(
        (name = "Authentication", description = "Auth related endpoints")
    )
)]
pub struct AuthApiDoc;