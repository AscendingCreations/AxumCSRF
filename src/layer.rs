use crate::CsrfConfig;
use axum::extract::Extension;
use cookie::Key;

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

    pub fn build() -> CsrfLayerBuilder {
        CsrfLayerBuilder::new()
    }
}

pub struct CsrfLayerBuilder {
    config: Option<CsrfConfig>,
    key: Option<Key>,
}

impl CsrfLayerBuilder {
    pub fn new() -> Self {
        Self {
            config: None,
            key: None,
        }
    }

    pub fn finish(self) -> Extension<CsrfLayer> {
        Extension(CsrfLayer {
            config: self.config.unwrap_or_else(CsrfConfig::default),
            key: self.key.unwrap_or_else(Key::generate),
        })
    }

    pub fn config(mut self, config: CsrfConfig) -> Self {
        self.config = Some(config);
        self
    }

    pub fn key(mut self, key: Key) -> Self {
        self.key = Some(key);
        self
    }
}
