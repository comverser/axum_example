use ::sea_orm::{Database, DatabaseConnection, DbErr};

pub mod repositories;
pub mod sea_orm;

pub async fn connect_database(database_url: &str) -> Result<DatabaseConnection, DbErr> {
    let database_connection = Database::connect(database_url).await?;

    Ok(database_connection)
}
