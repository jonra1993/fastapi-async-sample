use crate::models::{configuration::DatabaseSettings, User};
use sqlx::{postgres::PgPoolOptions, PgPool};

#[derive(Clone)]
pub struct PostgresService {
    pub connection: PgPool,
}

impl PostgresService {
    pub async fn new(con: Option<PgPool>) -> Self {
        if con.is_some() {
            return Self {
                connection: con.unwrap(),
            };
        }

        // Prepare the variables that the run method needs.
        let db_host = std::env::var("DATABASE_HOST").unwrap_or_else(|_| "database".to_string());
        let db_user = std::env::var("DATABASE_USER").unwrap_or_else(|_| "postgres".to_string());
        let db_password = std::env::var("DATABASE_PASSWORD").unwrap_or_else(|_| "postgres".to_string());
        let db_name = std::env::var("DATABASE_NAME").unwrap_or_else(|_| "fastapi_db".to_string());

        let db_port = std::env::var("DATABASE_PORT")
            .unwrap_or_else(|_| "5432".to_string())
            .parse()
            .expect("Failed to parse DB_PORT");
        let db_max_connections = std::env::var("DB_MAX_CONNECTIONS")
            .unwrap_or_else(|_| "10".to_owned())
            .parse()
            .expect("Failed to parse DB_MAX_CONNECTIONS");
        let dbSettings = DatabaseSettings::new(
            db_user,
            db_password,
            db_port,
            db_host,
            db_name,
            db_max_connections,
        );

        let connection = PgPoolOptions::new()
            .max_connections(dbSettings.max_connections)
            .connect(&dbSettings.get_connection_string_with_db())
            .await
            .expect("PgPoolOptions initialization failed");

        return Self { connection };
    }

    pub async fn getUsers(&self) -> Result<Vec<crate::models::User>, sqlx::Error> {
        let users: Vec<User> = sqlx::query_as("select * from \"User\"")
            .fetch_all(&self.connection)
            .await?;
        return Ok(users);
    }
}