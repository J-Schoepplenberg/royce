use bb8::RunError;
use thiserror::Error;
use tokio::task::{self};

#[derive(Error, Debug)]
pub enum AuthError {
    #[error(transparent)]
    PostgresRunError(#[from] RunError<tokio_postgres::Error>),

    #[error(transparent)]
    TokioPostgres(#[from] tokio_postgres::Error),

    #[error(transparent)]
    TaskJoin(#[from] task::JoinError),
}

#[derive(Error, Debug)]
pub enum RegisterError {
    #[error(transparent)]
    JoinError(#[from] task::JoinError),

    #[error(transparent)]
    TokioPostgres(#[from] tokio_postgres::Error),

    #[error(transparent)]
    PostgresRunError(#[from] RunError<tokio_postgres::Error>),
}
