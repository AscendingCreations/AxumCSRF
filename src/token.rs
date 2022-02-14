use crate::CsrfLayer;
use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    http::{self, StatusCode},
};
use bcrypt::{hash, verify};
use cookie::{Expiration, SameSite};
use rand::{distributions::Standard, Rng};
use tower_cookies::{Cookie, Cookies};

const BCRYPT_COST: u32 = 8;
///Failure Error when verification does not work or match.
pub struct VerificationFailure;

/// This is the Token that is generated when a user is routed to a page.
/// If a Cookie exists then it will be used as the Token.
/// Otherwise a new one is made.
#[derive(Debug, Clone)]
pub struct CsrfToken(String);

/// this auto pulls a Cookies nd Generates the CsrfToken from the extensions
#[async_trait]
impl<B> FromRequest<B> for CsrfToken
where
    B: Send,
{
    type Rejection = (http::StatusCode, &'static str);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let extensions = req.extensions().ok_or((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Can't extract CsrfToken: extensions has been taken by another extractor",
        ))?;

        let layer = extensions.get::<CsrfLayer>().cloned().ok_or((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Can't extract CsrfToken. Is `CSRFLayer` enabled?",
        ))?;

        let cookie_jar = extensions.get::<Cookies>().cloned().ok_or((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Can't extract Cookies. Is `tower_cookies::CookieManagerLayer` enabled?",
        ))?;

        let signed_cookies = cookie_jar.signed(&layer.key);

        //We check if the Cookie Exists as a signed Cookie or not. If so we use the value of the cookie.
        //If not we create a new one.
        if let Some(cookie) = signed_cookies.get(&layer.config.cookie_name) {
            Ok(CsrfToken(cookie.value().to_string()))
        } else {
            let values: Vec<u8> = rand::thread_rng()
                .sample_iter(Standard)
                .take(layer.config.cookie_len)
                .collect();
            let encoded = base64::encode(&values[..]);
            let mut now = time::OffsetDateTime::now_utc();
            now += layer.config.lifespan;

            let cookie = Cookie::build(layer.config.cookie_name.clone(), encoded.clone())
                .expires(Expiration::DateTime(now))
                .path("/")
                .secure(true)
                .same_site(SameSite::Strict)
                .http_only(true)
                .finish();

            signed_cookies.add(cookie);
            Ok(CsrfToken(encoded))
        }
    }
}

impl CsrfToken {
    ///Used to get the hashed Token to place within the form.
    pub fn authenticity_token(&self) -> String {
        hash(&self.0, BCRYPT_COST).unwrap()
    }

    ///Verifies that the form returned Token and the cookie tokens match.
    pub fn verify(&self, form_authenticity_token: &str) -> Result<(), VerificationFailure> {
        if verify(&self.0, form_authenticity_token).unwrap_or(false) {
            Ok(())
        } else {
            Err(VerificationFailure {})
        }
    }
}
