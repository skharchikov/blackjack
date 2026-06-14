use secrecy::{ExposeSecret, SecretString};
use subtle::ConstantTimeEq;

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
        self.expose()
            .as_bytes()
            .ct_eq(other.expose().as_bytes())
            .into()
    }
}

impl std::fmt::Debug for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[REDACTED]")
    }
}
