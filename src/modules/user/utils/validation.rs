use crate::modules::user::schemas::schemas::{
    RegisterUserRequest, LoginRequest, ChangePasswordRequest, UpdateProfileRequest
};
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

#[derive(Debug)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<ValidationError>,
}

impl ValidationResult {
    pub fn new() -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
        }
    }

    pub fn add_error(&mut self, field: String, message: String) {
        self.is_valid = false;
        self.errors.push(ValidationError { field, message });
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}

pub struct AuthValidator;

impl AuthValidator {
    /// Validate email format
    pub fn validate_email(email: &str) -> bool {
        let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        email_regex.is_match(email)
    }

    /// Validate password strength
    pub fn validate_password_strength(password: &str) -> (bool, Vec<String>) {
        let mut errors = Vec::new();
        
        if password.len() < 8 {
            errors.push("Password must be at least 8 characters long".to_string());
        }
        
        if password.len() > 128 {
            errors.push("Password must be no more than 128 characters long".to_string());
        }
        
        if !password.chars().any(|c| c.is_uppercase()) {
            errors.push("Password must contain at least one uppercase letter".to_string());
        }
        
        if !password.chars().any(|c| c.is_lowercase()) {
            errors.push("Password must contain at least one lowercase letter".to_string());
        }
        
        if !password.chars().any(|c| c.is_numeric()) {
            errors.push("Password must contain at least one number".to_string());
        }
        
        if !password.chars().any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c)) {
            errors.push("Password must contain at least one special character".to_string());
        }
        
        (errors.is_empty(), errors)
    }

    /// Validate name format
    pub fn validate_name(name: &str) -> (bool, Vec<String>) {
        let mut errors = Vec::new();
        
        if name.trim().is_empty() {
            errors.push("Name cannot be empty".to_string());
        }
        
        if name.len() < 2 {
            errors.push("Name must be at least 2 characters long".to_string());
        }
        
        if name.len() > 50 {
            errors.push("Name must be no more than 50 characters long".to_string());
        }
        
        if !name.chars().all(|c| c.is_alphabetic() || c.is_whitespace() || c == '-' || c == '\'') {
            errors.push("Name can only contain letters, spaces, hyphens, and apostrophes".to_string());
        }
        
        (errors.is_empty(), errors)
    }

    /// Validate registration request
    pub fn validate_registration(request: &RegisterUserRequest) -> ValidationResult {
        let mut result = ValidationResult::new();

        // Validate name
        let (name_valid, name_errors) = Self::validate_name(&request.name);
        if !name_valid {
            for error in name_errors {
                result.add_error("name".to_string(), error);
            }
        }

        // Validate email
        if !Self::validate_email(&request.email) {
            result.add_error("email".to_string(), "Invalid email format".to_string());
        }

        // Validate password
        let (password_valid, password_errors) = Self::validate_password_strength(&request.password);
        if !password_valid {
            for error in password_errors {
                result.add_error("password".to_string(), error);
            }
        }

        result
    }

    /// Validate login request
    pub fn validate_login(request: &LoginRequest) -> ValidationResult {
        let mut result = ValidationResult::new();

        // Validate email
        if !Self::validate_email(&request.email) {
            result.add_error("email".to_string(), "Invalid email format".to_string());
        }

        // Validate password (basic check)
        if request.password.is_empty() {
            result.add_error("password".to_string(), "Password cannot be empty".to_string());
        }

        result
    }

    /// Validate change password request
    pub fn validate_change_password(request: &ChangePasswordRequest) -> ValidationResult {
        let mut result = ValidationResult::new();

        // Validate current password
        if request.current_password.is_empty() {
            result.add_error("current_password".to_string(), "Current password cannot be empty".to_string());
        }

        // Validate new password
        let (password_valid, password_errors) = Self::validate_password_strength(&request.new_password);
        if !password_valid {
            for error in password_errors {
                result.add_error("new_password".to_string(), error);
            }
        }

        // Check if passwords are different
        if request.current_password == request.new_password {
            result.add_error("new_password".to_string(), "New password must be different from current password".to_string());
        }

        result
    }

    /// Validate update profile request
    pub fn validate_update_profile(request: &UpdateProfileRequest) -> ValidationResult {
        let mut result = ValidationResult::new();

        // Validate name if provided
        if let Some(name) = &request.name {
            let (name_valid, name_errors) = Self::validate_name(name);
            if !name_valid {
                for error in name_errors {
                    result.add_error("name".to_string(), error);
                }
            }
        }

        // Validate email if provided
        if let Some(email) = &request.email {
            if !Self::validate_email(email) {
                result.add_error("email".to_string(), "Invalid email format".to_string());
            }
        }

        // Check if at least one field is provided
        if request.name.is_none() && request.email.is_none() {
            result.add_error("general".to_string(), "At least one field must be provided for update".to_string());
        }

        result
    }

    /// Sanitize input string
    pub fn sanitize_string(input: &str) -> String {
        input.trim().to_string()
    }

    /// Check for common weak passwords
    pub fn is_weak_password(password: &str) -> bool {
        let weak_passwords = vec![
            "password", "123456", "123456789", "qwerty", "abc123",
            "password123", "admin", "letmein", "welcome", "monkey",
            "1234567890", "password1", "1234567", "dragon", "master"
        ];
        
        let password_lower = password.to_lowercase();
        weak_passwords.contains(&password_lower.as_str()) || password_lower.len() < 6
    }
}
