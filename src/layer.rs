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

    /// Creates a new CsrfLayerBuilder for more flexible setup.
    pub fn build() -> CsrfLayerBuilder {
        CsrfLayerBuilder::new()
    }
}

/// A builder to construct a CsrfLayer.
#[derive(Default)]
pub struct CsrfLayerBuilder {
    config: Option<CsrfConfig>,
    key: Option<Key>,
}

impl CsrfLayerBuilder {
    /// Creates the CsrfLayerBuilder with all fields set to None.
    pub fn new() -> Self {
        Self {
            config: None,
            key: None,
        }
    }

    /// Creates the CsrfLayer Extension from the currrent config.
    ///
    /// If any fields have not been set, this will construct default values for them.
    pub fn finish(self) -> Extension<CsrfLayer> {
        Extension(CsrfLayer {
            config: self.config.unwrap_or_default(),
            key: self.key.unwrap_or_else(Key::generate),
        })
    }

    /// Sets the CsrfConfig for the CsrfLayer.
    pub fn config(mut self, config: CsrfConfig) -> Self {
        self.config = Some(config);
        self
    }

    /// Sets the encryption key to use for private cookies.
    pub fn key(mut self, key: Key) -> Self {
        self.key = Some(key);
        self
    }
}
