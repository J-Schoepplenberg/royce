use crate::{database::setup::ConnectionPool, errors::RegisterError};
use axum_login::AuthUser;
use serde::{Deserialize, Serialize};
use tokio::task;
use tokio_postgres::Row;

/// User model.
///
/// Entity representing a user in the database.
#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
}

// User can be identified, authenticated, and authorized.
impl AuthUser for User {
    type Id = i32;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.password_hash.as_bytes()
    }
}

// User as a row of data returned from the database by a query.
impl From<Row> for User {
    fn from(value: Row) -> Self {
        Self {
            id: value.get("id"),
            username: value.get("username"),
            password_hash: value.get("password_hash"),
        }
    }
}

// User printed for debugging purposes.
impl std::fmt::Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("username", &self.username)
            .field("password", &"[redacted]")
            .finish()
    }
}

impl User {
    /// Register a new user in the database.
    pub async fn register(
        new_user: NewUser,
        db_pool: &ConnectionPool,
    ) -> Result<(), RegisterError> {
        // Offload the password hashing and salting to a blocking task.
        let hash =
            task::spawn_blocking(move || password_auth::generate_hash(&new_user.password)).await?;

        let db_connection = db_pool.get().await?;

        db_connection
            .execute(
                "INSERT INTO users (username, password_hash) VALUES ($1, $2)",
                &[&new_user.username, &hash],
            )
            .await?;

        tracing::info!("A new user signed up: {}.", new_user.username);

        Ok(())
    }

    /// Fetch a user by username from the database.
    pub async fn fetch_by_username(
        username: &str,
        db_pool: &ConnectionPool,
    ) -> Result<Option<Self>, RegisterError> {
        let db_connection = db_pool.get().await?;

        let row = db_connection
            .query_opt("SELECT * FROM users WHERE username = $1", &[&username])
            .await?;

        match row {
            Some(row) => Ok(Some(row.into())),
            None => Ok(None),
        }
    }
}

/// New user model for registration.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

/// Limited user model for responses.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserLimited {
    pub username: String,
}

// Convert a user into a limited user.
impl From<User> for UserLimited {
    fn from(user: User) -> Self {
        Self {
            username: user.username,
        }
    }
}

// Convert a new user into a limited user.
impl From<NewUser> for UserLimited {
    fn from(new_user: NewUser) -> Self {
        Self {
            username: new_user.username,
        }
    }
}
