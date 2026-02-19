use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct SessionInfo {
    pub user_id: Uuid,
    pub id: Uuid,
}
