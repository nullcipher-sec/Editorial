use crate::domain::AppUser;
use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Debug, Clone, Default, Serialize, Deserialize, Zeroize, ZeroizeOnDrop)]
pub struct AuthToken {
    pub value: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Zeroize, ZeroizeOnDrop)]
pub struct LoginCredentials {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub user: AppUser,
    pub token: Option<AuthToken>,
}

impl Session {
    pub fn new(user: AppUser, token: Option<String>) -> Self {
        Self {
            user,
            token: token.map(|v| AuthToken { value: v }),
        }
    }

    pub fn bearer_header(&self) -> Option<String> {
        self.token.as_ref().map(|t| format!("Bearer {}", t.value))
    }
}
