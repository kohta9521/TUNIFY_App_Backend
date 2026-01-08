use crate::domains::status::ApiStatus;

pub struct StatusService;

impl StatusService {
    pub fn new() -> Self {
        Self
    }

    pub fn get_status(&self) -> ApiStatus {
        ApiStatus {
            message: "API is running".to_string(),
        }
    }
}

