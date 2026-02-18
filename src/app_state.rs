use crate::{configuration::Settings, services::Services};
#[derive(Clone)]
pub struct AppState {
    pub services: Services,
    pub configuration: Settings,
}
