use time::Duration;

///This is the CSRF Config it is used to manage how we set the Restricted Cookie.
#[derive(Debug, Clone)]
pub struct CsrfConfig {
    /// CSRF Cookie lifespan
    pub(crate) lifespan: Duration,
    /// CSRF cookie name
    pub(crate) cookie_name: String,
    /// CSRF Token character length
    pub(crate) cookie_len: usize,
}

impl CsrfConfig {
    /// Set CSRF Cookie lifespan
    pub fn with_lifetime(mut self, time: Duration) -> Self {
        self.lifespan = time;
        self
    }

    /// Set CSRF cookie name
    pub fn with_cookie_name(mut self, name: &str) -> Self {
        self.cookie_name = name.into();
        self
    }

    /// Set CSRF Token character length
    pub fn with_cookie_len(mut self, length: usize) -> Self {
        self.cookie_len = length;
        self
    }
}

impl Default for CsrfConfig {
    fn default() -> Self {
        Self {
            /// Set to 6hour for default in Database Session stores.
            lifespan: Duration::hours(6),
            cookie_name: "Csrf_Token".into(),
            cookie_len: 16,
        }
    }
}
