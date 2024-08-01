use super::setup::get_db_client;
use anyhow::Result;
use tokio_postgres_migration::Migration;

/// Migrations that run when the server starts.
const MIGRATIONS_UP: [(&str, &str); 2] = [
    (
        "V1__create_users_table",
        include_str!("../../migrations/V1__create_users_table.sql"),
    ),
    (
        "V1__insert_user",
        include_str!("../../migrations/V1__insert_user.sql"),
    ),
];

/// Run the migrations.
pub async fn run_migrations() -> Result<()> {
    let mut client = get_db_client().await;
    let migration = Migration::new("migrations".to_string());
    migration.up(&mut client, &MIGRATIONS_UP).await?;
    Ok(())
}
