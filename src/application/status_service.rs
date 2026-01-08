use crate::domains::status::ApiStatus;

pub struct StatusService;

impl StatusService {
    pub fn new() -> Self {
        Self
    }

    pub fn get_status(&self) -> ApiStatus {
        ApiStatus {
            status: "ok".to_string(),
            message: "API is running".to_string(),
            status_code: 200,
        }
    }
}

