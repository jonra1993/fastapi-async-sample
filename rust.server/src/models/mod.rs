use uuid::Uuid;

pub mod configuration;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub framework: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

impl User {
    pub fn new(
        id: Uuid,
        framework: String,
        first_name: String,
        last_name: String,
        email: String,
    ) -> Self {
        Self {
            id,
            framework,
            first_name,
            last_name,
            email,
        }
    }
}