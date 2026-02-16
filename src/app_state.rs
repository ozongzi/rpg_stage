use sqlx::PgPool;
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub admin_email: String,
    pub deepseek_token: String,
}
