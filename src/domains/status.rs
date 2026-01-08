#[derive(Debug, Clone)]
pub struct ApiStatus {
    pub status: String,
    pub message: String,
    pub status_code: u16,
}