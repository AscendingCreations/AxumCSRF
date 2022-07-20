use crate::CsrfConfig;
use axum::extract::Extension;

/// CSRF layer struct used to pass key and CsrfConfig around.
#[derive(Clone)]
pub struct CsrfLayer {
    pub(crate) config: CsrfConfig,
}

impl CsrfLayer {
    /// Creates the CSRF Protection Layer.
    pub fn new(config: CsrfConfig) -> Extension<Self> {
        Extension(Self { config })
    }
}
