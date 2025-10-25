use crate::modules::user::model;
use crate::modules::user::repository::repository::UserRepository;
use crate::modules::user::schemas::schemas::{
    ChangePasswordRequest, ForgotPasswordRequest, LoginRequest, RefreshTokenRequest,
    RegisterUserRequest, ResetPasswordRequest, TokenResponse, UpdateProfileRequest, UserResponse,
};
use crate::modules::shared::security::token::{
    generate_access_token, generate_refresh_token,
};
use crate::modules::user::utils::hasher::{hash_password, verify_password};
use crate::modules::user::utils::validation::{validate_email, validate_password, validate_name};
use crate::modules::user::roles::UserRole;
use crate::modules::user::schemas::schemas::AdminCreateUserRequest;
use crate::errors::{AppError, AppResult};
use chrono::{FixedOffset, Utc};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use uuid::Uuid;

pub struct AuthService;

impl AuthService {
    /// Register a new user
    pub async fn register_user(
        db: &DatabaseConnection,
        new_user: &RegisterUserRequest,
    ) -> AppResult<UserResponse> {
        // Validate input
        validate_name(&new_user.name)?;
        validate_email(&new_user.email)?;
        validate_password(&new_user.password)?;

        // Check if user already exists
        if UserRepository::get_user_by_email(db, &new_user.email)
            .await
            .is_ok()
        {
            return Err(AppError::Validation(
                "User with this email already exists".to_string(),
            ));
        }

        let password = hash_password(&new_user.password);
        let user = UserRepository::insert_user(
            db, 
            &new_user.name, 
            &new_user.email, 
            &password, 
            UserRole::User.as_str()
        ).await?;

        // Convert to UserResponse
        Ok(UserResponse {
            id: user.id,
            name: user.name,
            email: user.email,
            role: user.role,
            is_active: user.is_active,
            created_at: user.created_at.with_timezone(&Utc),
            updated_at: user.updated_at.with_timezone(&Utc),
            latest_login: user.latest_login.map(|t| t.with_timezone(&Utc)),
        })
    }

    /// Admin create user
    pub async fn admin_create_user(
        db: &DatabaseConnection,
        new_user: &AdminCreateUserRequest,
    ) -> AppResult<UserResponse> {
        // Validate input
        validate_name(&new_user.name)?;
        validate_email(&new_user.email)?;
        validate_password(&new_user.password)?;

        // Validate role if provided
        let role = if let Some(role_str) = &new_user.role {
            let role = UserRole::from_str(role_str)
                .ok_or_else(|| AppError::Validation("Invalid role".to_string()))?;
            role.as_str()
        } else {
            UserRole::User.as_str()
        };

        // Check if user already exists
        if UserRepository::get_user_by_email(db, &new_user.email)
            .await
            .is_ok()
        {
            return Err(AppError::Validation(
                "User with this email already exists".to_string(),
            ));
        }

        let password = hash_password(&new_user.password);
        let user = UserRepository::insert_user(
            db, 
            &new_user.name, 
            &new_user.email, 
            &password, 
            role
        ).await?;

        // Convert to UserResponse
        Ok(UserResponse {
            id: user.id,
            name: user.name,
            email: user.email,
            role: user.role,
            is_active: user.is_active,
            created_at: user.created_at.with_timezone(&Utc),
            updated_at: user.updated_at.with_timezone(&Utc),
            latest_login: user.latest_login.map(|t| t.with_timezone(&Utc)),
        })
    }

    /// Get all users
    pub async fn get_all_users(
        db: &DatabaseConnection,
    ) -> AppResult<Vec<UserResponse>> {
        let users = UserRepository::fetch_all_users(db).await?;
        Ok(users
            .into_iter()
            .map(|user| UserResponse {
                id: user.id,
                name: user.name,
                email: user.email,
                role: user.role,
                is_active: user.is_active,
                created_at: user.created_at.with_timezone(&Utc),
                updated_at: user.updated_at.with_timezone(&Utc),
                latest_login: user.latest_login.map(|t| t.with_timezone(&Utc)),
            })
            .collect())
    }

    /// Login user and return tokens
    pub async fn login(
        db: &DatabaseConnection,
        login_request: &LoginRequest,
    ) -> AppResult<(TokenResponse, UserResponse)> {
        // Validate input
        validate_email(&login_request.email)?;
        
        let user = UserRepository::get_user_by_email(db, &login_request.email).await?;

        if verify_password(&login_request.password, &user.password) {
            let access_token = generate_access_token(&login_request.email)?;
            let refresh_token = generate_refresh_token();

            // Update latest login time
            let mut user_active: model::ActiveModel = user.clone().into();
            let now = Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap());
            user_active.latest_login = Set(Some(now));
            user_active.update(db).await?;

            let token_response = TokenResponse {
                access_token,
                refresh_token,
                token_type: "Bearer".to_string(),
                expires_in: 900, // 15 minutes
            };

            let user_response = UserResponse {
                id: user.id,
                name: user.name,
                email: user.email,
                role: user.role,
                is_active: user.is_active,
                created_at: user.created_at.with_timezone(&Utc),
                updated_at: user.updated_at.with_timezone(&Utc),
                latest_login: user.latest_login.map(|t| t.with_timezone(&Utc)),
            };

            Ok((token_response, user_response))
        } else {
            Err(AppError::Authentication("Invalid credentials".to_string()))
        }
    }

    /// Refresh access token
    pub async fn refresh_token(
        _db: &DatabaseConnection,
        _refresh_request: &RefreshTokenRequest,
    ) -> AppResult<TokenResponse> {
        // In a real implementation, you would validate the refresh token
        // and get the user information from it
        // For now, we'll generate a new token
        let access_token = generate_access_token("user@example.com")?;
        let refresh_token = generate_refresh_token();

        Ok(TokenResponse {
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: 900,
        })
    }

    /// Logout user (invalidate tokens)
    pub async fn logout(_db: &DatabaseConnection, _user_id: Uuid) -> Result<(), sea_orm::DbErr> {
        // In a real implementation, you would invalidate the tokens
        // by storing them in a blacklist or updating user status
        Ok(())
    }

    /// Change user password
    pub async fn change_password(
        db: &DatabaseConnection,
        user_id: Uuid,
        change_request: &ChangePasswordRequest,
    ) -> Result<(), sea_orm::DbErr> {
        let user = UserRepository::get_user_by_id(db, user_id).await?;

        if !verify_password(&change_request.current_password, &user.password) {
            return Err(sea_orm::DbErr::Custom(
                "Current password is incorrect".to_string(),
            ));
        }

        let new_password = hash_password(&change_request.new_password);
        let mut user_active: model::ActiveModel = user.into();
        user_active.password = Set(new_password);
        let now = Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap());
        user_active.updated_at = Set(now);
        user_active.update(db).await?;

        Ok(())
    }

    /// Update user profile
    pub async fn update_profile(
        db: &DatabaseConnection,
        user_id: Uuid,
        update_request: &UpdateProfileRequest,
    ) -> Result<UserResponse, sea_orm::DbErr> {
        let user = UserRepository::get_user_by_id(db, user_id).await?;
        let mut user_active: model::ActiveModel = user.clone().into();

        if let Some(name) = &update_request.name {
            user_active.name = Set(name.clone());
        }
        if let Some(email) = &update_request.email {
            // Check if email is already taken by another user
            if email != &user.email {
                if UserRepository::get_user_by_email(db, email).await.is_ok() {
                    return Err(sea_orm::DbErr::Custom("Email already taken".to_string()));
                }
                user_active.email = Set(email.clone());
            }
        }
        let now = Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap());
        user_active.updated_at = Set(now);

        let updated_user = user_active.update(db).await?;

        Ok(UserResponse {
            id: updated_user.id,
            name: updated_user.name,
            email: updated_user.email,
            role: updated_user.role,
            is_active: updated_user.is_active,
            created_at: user.created_at.with_timezone(&Utc),
            updated_at: user.updated_at.with_timezone(&Utc),
            latest_login: updated_user.latest_login.map(|t| t.with_timezone(&Utc)),
        })
    }

    /// Get user by email
    pub async fn get_user_by_email(
        db: &DatabaseConnection,
        email: &str,
    ) -> Result<UserResponse, sea_orm::DbErr> {
        let user = UserRepository::get_user_by_email(db, email).await?;
        Ok(UserResponse {
            id: user.id,
            name: user.name,
            email: user.email,
            role: user.role,
            is_active: user.is_active,
            created_at: user.created_at.with_timezone(&Utc),
            updated_at: user.updated_at.with_timezone(&Utc),
            latest_login: user.latest_login.map(|t| t.with_timezone(&Utc)),
        })
    }

    /// Get user by ID
    pub async fn get_user_by_id(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<UserResponse, sea_orm::DbErr> {
        let user = UserRepository::get_user_by_id(db, id).await?;
        Ok(UserResponse {
            id: user.id,
            name: user.name,
            email: user.email,
            role: user.role,
            is_active: user.is_active,
            created_at: user.created_at.with_timezone(&Utc),
            updated_at: user.updated_at.with_timezone(&Utc),
            latest_login: user.latest_login.map(|t| t.with_timezone(&Utc)),
        })
    }

    /// Forgot password (send reset email)
    pub async fn forgot_password(
        db: &DatabaseConnection,
        forgot_request: &ForgotPasswordRequest,
    ) -> Result<(), sea_orm::DbErr> {
        // Check if user exists
        let _user = UserRepository::get_user_by_email(db, &forgot_request.email).await?;

        // In a real implementation, you would:
        // 1. Generate a reset token
        // 2. Store it in database with expiration
        // 3. Send email with reset link

        Ok(())
    }

    /// Reset password with token
    pub async fn reset_password(
        _db: &DatabaseConnection,
        _reset_request: &ResetPasswordRequest,
    ) -> Result<(), sea_orm::DbErr> {
        // In a real implementation, you would:
        // 1. Validate the reset token
        // 2. Check if token is not expired
        // 3. Update user password
        // 4. Invalidate the token

        Ok(())
    }
}
