use crate::database::models::user::UserLimited;
use serde::Serialize;

// Note: we are using #[allow(non_snake_case)] to match the JavaScript object keys.

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct StoreApiResponse {
    isAuthenticated: bool,
    user: Option<UserLimited>,
}

impl StoreApiResponse {
    #[allow(non_snake_case)]
    pub fn new(isAuthenticated: bool, user: Option<UserLimited>) -> Self {
        Self {
            isAuthenticated,
            user,
        }
    }
}
