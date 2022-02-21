pub enum GrantType {
    Bearer,
    Authorization,
    Refresh,
}

impl std::fmt::Display for GrantType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Authorization => write!(f, "authorization_code"),
            Self::Bearer => write!(f, "bearer"),
            Self::Refresh => write!(f, "refresh_token"),
        }
    }
}