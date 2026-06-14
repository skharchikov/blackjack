use secrecy::{ExposeSecret, SecretString};

/// Opaque password wrapper. Never printed or logged.
#[derive(Clone)]
pub struct Password(SecretString);

impl Password {
    pub fn new(s: String) -> Self {
        Self(SecretString::from(s))
    }

    pub fn expose(&self) -> &str {
        self.0.expose_secret()
    }
}

impl PartialEq for Password {
    fn eq(&self, other: &Self) -> bool {
        self.expose() == other.expose()
    }
}

impl std::fmt::Debug for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[REDACTED]")
    }
}
