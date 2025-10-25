use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserRole {
    Admin,
    User,
    Moderator,
}

impl UserRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            UserRole::Admin => "admin",
            UserRole::User => "user",
            UserRole::Moderator => "moderator",
        }
    }

    pub fn from_str(role: &str) -> Option<Self> {
        match role {
            "admin" => Some(UserRole::Admin),
            "user" => Some(UserRole::User),
            "moderator" => Some(UserRole::Moderator),
            _ => None,
        }
    }

    pub fn is_admin(&self) -> bool {
        matches!(self, UserRole::Admin)
    }

    pub fn can_create_users(&self) -> bool {
        matches!(self, UserRole::Admin)
    }

    pub fn can_manage_users(&self) -> bool {
        matches!(self, UserRole::Admin | UserRole::Moderator)
    }
}

impl Default for UserRole {
    fn default() -> Self {
        UserRole::User
    }
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
