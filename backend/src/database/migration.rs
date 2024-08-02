use super::setup::get_db_client;
use anyhow::Result;
use tokio_postgres::Client;
use tokio_postgres_migration::Migration;

/// Migrations that run when the server starts.
const MIGRATIONS_UP: [(&str, &str); 1] = [(
    "V1__create_users_table",
    include_str!("../../migrations/V1__create_users_table.sql"),
)];

/// Run migrations.
pub async fn run_migrations() -> Result<()> {
    let mut client = get_db_client().await;

    let already_present = migrations_present(&mut client).await?;

    if !already_present {
        let migration = Migration::new("migrations".to_string());
        migration.up(&mut client, &MIGRATIONS_UP).await?;
    }

    Ok(())
}

/// Check if the migrations table is present.
async fn migrations_present(client: &mut Client) -> Result<bool> {
    let query = r#"
        SELECT EXISTS (
            SELECT 1
            FROM information_schema.tables
            WHERE table_schema = 'public'
            AND table_name = 'migrations'
        );
    "#;
    let row = client.query_one(query, &[]).await?;
    let exists: bool = row.get(0);
    Ok(exists)
}
