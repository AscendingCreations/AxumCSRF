use crate::CsrfConfig;
use http::Request;
use std::task::{Context, Poll};
use tower_service::Service;

#[derive(Clone)]
pub struct AxumCsrfService<S> {
    pub(crate) config: CsrfConfig,
    pub(crate) inner: S,
}

impl<ResBody, S> Service<Request<ResBody>> for AxumCsrfService<S>
where
    S: Service<Request<ResBody>>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    #[inline]
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<ResBody>) -> Self::Future {
        req.extensions_mut().insert(self.config.clone());
        self.inner.call(req)
    }
}
