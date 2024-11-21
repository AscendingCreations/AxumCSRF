use crate::{cookies::*, CsrfConfig, CsrfError};
use async_trait::async_trait;
#[cfg(not(feature = "layer"))]
use axum_core::extract::FromRef;
use axum_core::{
    extract::FromRequestParts,
    response::{IntoResponse, IntoResponseParts, Response, ResponseParts},
};
use cookie::{Cookie, CookieJar, Expiration};
use http::{self, request::Parts};
use std::convert::Infallible;

use base64ct::{Base64, Encoding};
use hmac::{Hmac, Mac};
use sha2::Sha256;

/// This is the Token that is generated when a user is routed to a page.
/// If a Cookie exists then it will be used as the Token.
/// Otherwise a new one is made.
#[derive(Clone)]
pub struct CsrfToken {
    pub(crate) token: String,
    pub(crate) config: CsrfConfig,
}

/// this auto pulls a Cookies nd Generates the CsrfToken from the extensions
#[cfg(not(feature = "layer"))]
#[async_trait]
impl<S> FromRequestParts<S> for CsrfToken
where
    S: Send + Sync,
    CsrfConfig: FromRef<S>,
{
    type Rejection = (http::StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let config = CsrfConfig::from_ref(state);
        let token = get_token(&config, &mut parts.headers);

        Ok(CsrfToken { token, config })
    }
}

#[cfg(feature = "layer")]
#[async_trait]
impl<S> FromRequestParts<S> for CsrfToken
where
    S: Send + Sync,
{
    type Rejection = (http::StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let token = parts.extensions.get::<CsrfToken>().cloned().ok_or((
            http::StatusCode::INTERNAL_SERVER_ERROR,
            "Can't extract CsrfConfig. Is `CSRFLayer` enabled?",
        ))?;

        Ok(token)
    }
}

impl IntoResponseParts for CsrfToken {
    type Error = Infallible;

    fn into_response_parts(self, mut res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        let mut jar = CookieJar::new();
        let lifespan = time::OffsetDateTime::now_utc() + self.config.lifespan;

        let mut cookie_builder = Cookie::build((
            if self.config.prefix_with_host {
                let mut prefixed = "__Host-".to_owned();
                prefixed.push_str(&self.config.cookie_name);
                prefixed
            } else {
                self.config.cookie_name.clone()
            },
            self.token.clone(),
        ))
        .path(self.config.cookie_path.clone())
        .secure(self.config.cookie_secure)
        .http_only(self.config.cookie_http_only)
        .same_site(self.config.cookie_same_site);

        if self.config.lifespan > time::Duration::seconds(0) {
            cookie_builder = cookie_builder.expires(Expiration::DateTime(lifespan));
        }

        if let Some(domain) = &self.config.cookie_domain {
            cookie_builder = cookie_builder.domain(domain.clone());
        }

        jar.add_cookie(cookie_builder.build(), &self.config.key);

        set_cookies(jar, res.headers_mut());
        Ok(res)
    }
}

impl IntoResponse for CsrfToken {
    fn into_response(self) -> Response {
        (self, ()).into_response()
    }
}

impl CsrfToken {
    ///Used to get the hashed Token to place within the form.
    pub fn authenticity_token(&self) -> Result<String, crate::CsrfError> {
        let mut mac = Hmac::<Sha256>::new_from_slice(self.config.salt.as_bytes())
            .map_err(|_| CsrfError::Salt)?;
        mac.update(self.token.as_bytes());

        let result = mac.finalize();
        let bytes = result.into_bytes();
        Ok(Base64::encode_string(&bytes))
    }

    ///Verifies that the form returned Token and the cookie tokens match.
    pub fn verify(&self, form_authenticity_token: &str) -> Result<(), crate::CsrfError> {
        let mut mac = Hmac::<Sha256>::new_from_slice(self.config.salt.as_bytes())
            .map_err(|_| CsrfError::Salt)?;
        mac.update(self.token.as_bytes());

        mac.verify_slice(
            &Base64::decode_vec(form_authenticity_token).map_err(|_| CsrfError::PasswordHash)?,
        )
        .map_err(|_| CsrfError::Verify)?;
        Ok(())
    }
}
