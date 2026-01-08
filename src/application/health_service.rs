#[derive(Debug, Clone)]
pub struct HealthStatus {
    pub status: String,
}

pub struct HealthService;

impl HealthService {
    pub fn new() -> Self {
        Self
    }

    pub fn get_health(&self) -> HealthStatus {
        HealthStatus {
            status: "healthy".to_string(),
        }
    }
}