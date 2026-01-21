#[derive(Debug, Clone)]
pub struct LoginState {
    pub username: String,
    pub status: LoginStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoginStatus {
    Idle,
    Connecting,
    Error(String),
}

impl Default for LoginState {
    fn default() -> Self {
        Self {
            username: String::new(),
            status: LoginStatus::Idle,
        }
    }
}
