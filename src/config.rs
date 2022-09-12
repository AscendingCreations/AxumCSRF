pub use cookie::{Key, SameSite};
use std::borrow::Cow;
use time::Duration;

///This is the CSRF Config it is used to manage how we set the Restricted Cookie.
#[derive(Clone)]
pub struct CsrfConfig {
    /// CSRF Cookie lifespan
    pub(crate) lifespan: Duration,
    /// CSRF cookie name
    pub(crate) cookie_name: String,
    /// CSRF Token character length
    pub(crate) cookie_len: usize,
    /// Session cookie domain
    pub(crate) cookie_domain: Option<Cow<'static, str>>,
    /// Session cookie http only flag
    pub(crate) cookie_http_only: bool,
    /// Session cookie http only flag
    pub(crate) cookie_path: Cow<'static, str>,
    /// Resticts how Cookies are sent cross-site. Default is `SameSite::None`
    /// Only works if domain is also set.
    pub(crate) cookie_same_site: SameSite,
    /// Session cookie secure flag
    pub(crate) cookie_secure: bool,
    ///Encyption Key used to encypt cookies for confidentiality, integrity, and authenticity.
    pub(crate) key: Option<Key>,
}

impl std::fmt::Debug for CsrfConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CsrfConfig")
            .field("lifespan", &self.lifespan)
            .field("cookie_name", &self.cookie_name)
            .field("cookie_len", &self.cookie_len)
            .field("cookie_domain", &self.cookie_domain)
            .field("cookie_http_only", &self.cookie_http_only)
            .field("cookie_path", &self.cookie_path)
            .field("cookie_same_site", &self.cookie_same_site)
            .field("cookie_secure", &self.cookie_secure)
            .field("key", &"key hidden")
            .finish()
    }
}

impl CsrfConfig {
    /// Creates [`Default`] configuration of [`CsrfConfig`].
    /// This is equivalent to the [`CsrfConfig::default()`].
    #[must_use]
    pub fn new() -> Self {
        Default::default()
    }

    /// Set's the csrf's cookie's domain name.
    ///
    /// # Examples
    /// ```rust
    /// use axum_csrf::CsrfConfig;
    ///
    /// let config = CsrfConfig::default().with_cookie_domain(Some("www.helpme.com".to_string()));
    /// ```
    ///
    #[must_use]
    pub fn with_cookie_domain(mut self, name: impl Into<Option<Cow<'static, str>>>) -> Self {
        self.cookie_domain = name.into();
        self
    }

    /// Set's the csrf's lifetime (expiration time).
    ///
    /// # Examples
    /// ```rust
    /// use axum_csrf::CsrfConfig;
    /// use chrono::Duration;
    ///
    /// let config = CsrfConfig::default().with_lifetime(Duration::days(32));
    /// ```
    ///
    #[must_use]
    pub fn with_lifetime(mut self, time: Duration) -> Self {
        self.lifespan = time;
        self
    }

    /// Set's the csrf's cookie's name.
    ///
    /// # Examples
    /// ```rust
    /// use axum_csrf::CsrfConfig;
    ///
    /// let config = CsrfConfig::default().with_cookie_name("my_cookie");
    /// ```
    ///
    #[must_use]
    pub fn with_cookie_name(mut self, name: &str) -> Self {
        self.cookie_name = name.into();
        self
    }

    /// Set's the csrf's cookie's path.
    ///
    /// This is used to deturmine when the cookie takes effect within the website path.
    /// Leave as default ("/") for cookie to be used site wide.
    ///
    /// # Examples
    /// ```rust
    /// use axum_csrf::CsrfConfig;
    ///
    /// let config = CsrfConfig::default().with_cookie_path("/");
    /// ```
    ///
    #[must_use]
    pub fn with_cookie_path(mut self, path: impl Into<Cow<'static, str>>) -> Self {
        self.cookie_path = path.into();
        self
    }

    /// Set's the csrf's cookie's Same Site Setting for Cross-Site restrictions.
    ///
    /// Only works if Domain is also set to restrict it to that domain only.
    ///
    /// # Examples
    /// ```rust
    /// use axum_csrf::CsrfConfig;
    /// use cookie::SameSite;
    ///
    /// let config = CsrfConfig::default().with_cookie_same_site(SameSite::Strict);
    /// ```
    ///
    #[must_use]
    pub fn with_cookie_same_site(mut self, same_site: SameSite) -> Self {
        self.cookie_same_site = same_site;
        self
    }

    /// Set's the csrf's cookie's to http only.
    ///
    /// # Examples
    /// ```rust
    /// use axum_csrf::CsrfConfig;
    ///
    /// let config = CsrfConfig::default().with_http_only(false);
    /// ```
    ///
    #[must_use]
    pub fn with_http_only(mut self, is_set: bool) -> Self {
        self.cookie_http_only = is_set;
        self
    }

    /// Set's the csrf's secure flag for if it gets sent over https.
    ///
    /// # Examples
    /// ```rust
    /// use axum_csrf::CsrfConfig;
    ///
    /// let config = CsrfConfig::default().with_secure(true);
    /// ```
    ///
    #[must_use]
    pub fn with_secure(mut self, is_set: bool) -> Self {
        self.cookie_secure = is_set;
        self
    }

    /// Set's the csrf's token length.
    ///
    /// # Examples
    /// ```rust
    /// use axum_csrf::CsrfConfig;
    ///
    /// let config = CsrfConfig::default().with_cookie_len(16);
    /// ```
    ///
    #[must_use]
    pub fn with_cookie_len(mut self, length: usize) -> Self {
        self.cookie_len = length;
        self
    }

    /// Set's the csrf's cookie encyption key enabling private cookies.
    ///
    /// When Set it will enforce Private cookies across all Sessions.
    /// If you use Key::generate() it will make a new key each server reboot.
    /// To prevent this make and save a key to a config file for long term usage.
    /// For Extra Security Regenerate the key every so many months to a year.
    ///
    /// # Examples
    /// ```rust
    /// use axum_csrf::{Key, CsrfConfig};
    ///
    /// let config = CsrfConfig::default().with_key(Key::generate());
    /// ```
    ///
    #[must_use]
    pub fn with_key(mut self, key: Option<Key>) -> Self {
        self.key = key;
        self
    }
}

impl Default for CsrfConfig {
    fn default() -> Self {
        Self {
            /// Set to 6hour for default in Database Session stores.
            lifespan: Duration::hours(6),
            cookie_name: "Csrf_Token".into(),
            cookie_path: "/".into(),
            cookie_http_only: true,
            cookie_secure: false,
            cookie_domain: None,
            cookie_same_site: SameSite::Lax,
            cookie_len: 16,
            //We do this by default since we always want this to be secure.
            key: Some(Key::generate()),
        }
    }
}
