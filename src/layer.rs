use crate::CsrfConfig;
use axum::extract::Extension;
use tower_cookies::Key;

/// CSRF layer struct used to pass key and CsrfConfig around.
#[derive(Clone)]
pub struct CsrfLayer {
    pub(crate) config: CsrfConfig,
    ///we will generate the key only when we start a new layer instances.
    pub(crate) key: Key,
}

impl CsrfLayer {
    /// Creates the CSRF Protection Layer.
    pub fn new(config: CsrfConfig) -> Extension<Self> {
        Extension(Self {
            config,
            key: Key::generate(),
        })
    }
}
