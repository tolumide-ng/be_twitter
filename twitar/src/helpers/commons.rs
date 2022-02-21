#[derive(Debug, derive_more::Display)]
pub enum GrantType {
    #[display(fmt = "bearer")]
    Bearer,
    #[display(fmt = "authorization_code")]
    Authorization,
    #[display(fmt = "refresh_token")]
    Refresh,
}
