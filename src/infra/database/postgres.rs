use sea_orm::{Database, DatabaseConnection};

pub type DbPool = DatabaseConnection;

pub async fn establish_connection(database_url: String) -> DbPool {
    Database::connect(&database_url)
        .await
        .expect("Cannot connect to database")
}
