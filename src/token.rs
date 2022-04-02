use crate::CsrfLayer;
use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    http::{
        self,
        header::{COOKIE, SET_COOKIE},
        HeaderMap, StatusCode,
    },
};
use bcrypt::{hash, verify};
use cookie::{Cookie, CookieJar, Expiration, Key, SameSite};
use rand::{distributions::Standard, Rng};

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
        let layer = req.extensions().get::<CsrfLayer>().cloned().ok_or((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Can't extract CsrfToken. Is `CSRFLayer` enabled?",
        ))?;

        let mut cookie_jar = get_cookies(req, &layer.key);

        let mut private_jar = cookie_jar.private_mut(&layer.key);

        //We check if the Cookie Exists as a signed Cookie or not. If so we use the value of the cookie.
        //If not we create a new one.
        if let Some(cookie) = private_jar.get(&layer.config.cookie_name) {
            let plain_cookie = match private_jar.decrypt(cookie) {
                Some(v) => v,
                None => return Err((StatusCode::INTERNAL_SERVER_ERROR, "Can't decrypt cookie?")),
            };

            Ok(CsrfToken(plain_cookie.value().to_string()))
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

            private_jar.add(cookie);

            set_cookies(cookie_jar, req.headers_mut());
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

fn get_cookies<B>(req: &RequestParts<B>, key: &Key) -> CookieJar {
    let mut jar = CookieJar::new();
    let mut private_jar = jar.private_mut(key);
    let cookie_iter = req
        .headers()
        .get_all(COOKIE)
        .into_iter()
        .filter_map(|value| value.to_str().ok())
        .flat_map(|value| value.split(';'))
        .filter_map(|cookie| Cookie::parse_encoded(cookie.to_owned()).ok());

    for cookie in cookie_iter {
        if let Some(cookie) = private_jar.decrypt(cookie) {
            private_jar.add_original(cookie);
        }
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
