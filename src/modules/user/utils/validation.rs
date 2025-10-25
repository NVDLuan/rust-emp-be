use validator::{Validate, ValidationError};
use regex::Regex;
use crate::errors::{AppError, AppResult};

#[derive(Validate)]
pub struct EmailValidation {
    #[validate(email)]
    pub email: String,
}

#[derive(Validate)]
pub struct PasswordValidation {
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    #[validate(custom(function = "validate_password_strength"))]
    pub password: String,
}

pub fn validate_email(email: &str) -> AppResult<()> {
    let email_validation = EmailValidation {
        email: email.to_string(),
    };
    
    email_validation.validate()
        .map_err(|e| AppError::Validation(format!("Email validation failed: {}", e)))?;
    
    Ok(())
}

pub fn validate_password(password: &str) -> AppResult<()> {
    let password_validation = PasswordValidation {
        password: password.to_string(),
    };
    
    password_validation.validate()
        .map_err(|e| AppError::Validation(format!("Password validation failed: {}", e)))?;
    
    Ok(())
}

fn validate_password_strength(password: &str) -> Result<(), ValidationError> {
    let mut has_upper = false;
    let mut has_lower = false;
    let mut has_digit = false;
    let mut has_special = false;
    
    for ch in password.chars() {
        if ch.is_uppercase() {
            has_upper = true;
        } else if ch.is_lowercase() {
            has_lower = true;
        } else if ch.is_ascii_digit() {
            has_digit = true;
        } else if ch.is_ascii_punctuation() {
            has_special = true;
        }
    }
    
    if !has_upper {
        return Err(ValidationError::new("password_must_contain_uppercase"));
    }
    if !has_lower {
        return Err(ValidationError::new("password_must_contain_lowercase"));
    }
    if !has_digit {
        return Err(ValidationError::new("password_must_contain_digit"));
    }
    if !has_special {
        return Err(ValidationError::new("password_must_contain_special_char"));
    }
    
    Ok(())
}

pub fn validate_name(name: &str) -> AppResult<()> {
    if name.trim().is_empty() {
        return Err(AppError::Validation("Name cannot be empty".to_string()));
    }
    
    if name.len() < 2 {
        return Err(AppError::Validation("Name must be at least 2 characters long".to_string()));
    }
    
    if name.len() > 100 {
        return Err(AppError::Validation("Name must be less than 100 characters".to_string()));
    }
    
    // Check for valid characters (letters, spaces, hyphens, apostrophes)
    let name_regex = Regex::new(r"^[a-zA-Z\s\-']+$")
        .map_err(|_| AppError::Internal("Invalid regex pattern".to_string()))?;
    
    if !name_regex.is_match(name) {
        return Err(AppError::Validation("Name contains invalid characters".to_string()));
    }
    
    Ok(())
}