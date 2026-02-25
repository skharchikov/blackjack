#[derive(Debug, Clone)]
pub struct LoginState {
    pub username: String,
    pub password: String,
    pub active_field: LoginField,
    pub status: LoginStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoginField {
    Username,
    Password,
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
            password: String::new(),
            active_field: LoginField::Username,
            status: LoginStatus::Idle,
        }
    }
}
