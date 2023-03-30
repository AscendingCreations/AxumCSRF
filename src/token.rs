use crate::CsrfConfig;
use async_trait::async_trait;
use axum_core::{
    extract::{FromRef, FromRequestParts},
    response::{IntoResponse, IntoResponseParts, Response, ResponseParts},
};
use base64::Engine;
use bcrypt::{hash, verify, BASE_64};
use cookie::{Cookie, CookieJar, Expiration, Key};
use http::{
    self,
    header::{COOKIE, SET_COOKIE},
    request::Parts,
    HeaderMap,
};
use rand::{distributions::Standard, Rng};
use std::convert::Infallible;

const BCRYPT_COST: u32 = 8;
///Failure Error when verification does not work or match.
pub struct VerificationFailure;

/// This is the Token that is generated when a user is routed to a page.
/// If a Cookie exists then it will be used as the Token.
/// Otherwise a new one is made.
#[derive(Clone)]
pub struct CsrfToken {
    token: String,
    config: CsrfConfig,
}

/// this auto pulls a Cookies nd Generates the CsrfToken from the extensions
#[async_trait]
impl<S> FromRequestParts<S> for CsrfToken
where
    S: Send + Sync,
    CsrfConfig: FromRef<S>,
{
    type Rejection = (http::StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let config = CsrfConfig::from_ref(state);
        let cookie_jar = get_cookies(&mut parts.headers);

        //We check if the Cookie Exists as a signed Cookie or not. If so we use the value of the cookie.
        //If not we create a new one.
        if let Some(cookie) = cookie_jar.get_cookie(&config.cookie_name, &config.key) {
            Ok(CsrfToken {
                token: cookie.value().to_string(),
                config,
            })
        } else {
            let values: Vec<u8> = rand::thread_rng()
                .sample_iter(Standard)
                .take(config.cookie_len)
                .collect();

            Ok(CsrfToken {
                token: BASE_64.encode(&values[..]),
                config,
            })
        }
    }
}

impl IntoResponseParts for CsrfToken {
    type Error = Infallible;

    fn into_response_parts(self, mut res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        let mut jar = CookieJar::new();
        let lifespan = time::OffsetDateTime::now_utc() + self.config.lifespan;

        let mut cookie_builder = Cookie::build(self.config.cookie_name.clone(), self.token.clone())
            .path(self.config.cookie_path.clone())
            .secure(self.config.cookie_secure)
            .http_only(self.config.cookie_http_only)
            .same_site(self.config.cookie_same_site)
            .expires(Expiration::DateTime(lifespan));

        if let Some(domain) = &self.config.cookie_domain {
            cookie_builder = cookie_builder.domain(domain.clone());
        }

        jar.add_cookie(cookie_builder.finish(), &self.config.key);

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
    pub fn authenticity_token(&self) -> String {
        hash(&self.token, BCRYPT_COST).unwrap()
    }

    ///Verifies that the form returned Token and the cookie tokens match.
    pub fn verify(&self, form_authenticity_token: &str) -> Result<(), VerificationFailure> {
        if verify(&self.token, form_authenticity_token).unwrap_or(false) {
            Ok(())
        } else {
            Err(VerificationFailure {})
        }
    }
}

pub(crate) trait CookiesExt {
    fn get_cookie(&self, name: &str, key: &Option<Key>) -> Option<Cookie<'static>>;
    fn add_cookie(&mut self, cookie: Cookie<'static>, key: &Option<Key>);
}

impl CookiesExt for CookieJar {
    fn get_cookie(&self, name: &str, key: &Option<Key>) -> Option<Cookie<'static>> {
        if let Some(key) = key {
            self.private(key).get(name)
        } else {
            self.get(name).cloned()
        }
    }

    fn add_cookie(&mut self, cookie: Cookie<'static>, key: &Option<Key>) {
        if let Some(key) = key {
            self.private_mut(key).add(cookie)
        } else {
            self.add(cookie)
        }
    }
}

fn get_cookies(headers: &mut HeaderMap) -> CookieJar {
    let mut jar = CookieJar::new();

    let cookie_iter = headers
        .get_all(COOKIE)
        .into_iter()
        .filter_map(|value| value.to_str().ok())
        .flat_map(|value| value.split(';'))
        .filter_map(|cookie| Cookie::parse_encoded(cookie.to_owned()).ok());

    for cookie in cookie_iter {
        jar.add_original(cookie);
    }

    jar
}

fn set_cookies(jar: CookieJar, headers: &mut HeaderMap) {
    for cookie in jar.delta() {
        if let Ok(header_value) = cookie.encoded().to_string().parse() {
            headers.append(SET_COOKIE, header_value);
        }
    }
}
