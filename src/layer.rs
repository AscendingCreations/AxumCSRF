use crate::{AxumCsrfService, CsrfConfig};
use tower_layer::Layer;

/// CSRF layer struct used to pass key and CsrfConfig around.
#[derive(Clone)]
pub struct CsrfLayer {
    pub(crate) config: CsrfConfig,
}

impl CsrfLayer {
    /// Creates the CSRF Protection Layer.
    pub fn new(config: CsrfConfig) -> Self {
        Self { config }
    }
}

impl<S> Layer<S> for CsrfLayer {
    type Service = AxumCsrfService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AxumCsrfService {
            config: self.config.clone(),
            inner,
        }
    }
}
