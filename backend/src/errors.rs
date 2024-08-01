use bb8::RunError;
use thiserror::Error;
use tokio::task::{self};
use tokio_postgres::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error(transparent)]
    RunError(#[from] RunError<tokio_postgres::Error>),

    #[error(transparent)]
    PostgreSQL(#[from] Error),

    #[error(transparent)]
    TaskJoin(#[from] task::JoinError),
}

#[derive(Error, Debug)]
pub enum RegisterError {
    #[error(transparent)]
    JoinError(#[from] task::JoinError),

    #[error(transparent)]
    PostgresError(#[from] tokio_postgres::Error),

    #[error(transparent)]
    RunError(#[from] RunError<tokio_postgres::Error>),
}