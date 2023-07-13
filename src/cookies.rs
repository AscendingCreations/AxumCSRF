use crate::CsrfConfig;
use cookie::{Cookie, CookieJar, Key};
use http::{
    self,
    header::{COOKIE, SET_COOKIE},
    HeaderMap,
};
use rand::{distributions::Alphanumeric, thread_rng, Rng};

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

pub(crate) fn get_cookies(headers: &mut HeaderMap) -> CookieJar {
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

pub(crate) fn set_cookies(jar: CookieJar, headers: &mut HeaderMap) {
    for cookie in jar.delta() {
        if let Ok(header_value) = cookie.encoded().to_string().parse() {
            headers.append(SET_COOKIE, header_value);
        }
    }
}

pub(crate) fn get_token(config: &CsrfConfig, headers: &mut HeaderMap) -> String {
    let cookie_jar = get_cookies(headers);

    //We check if the Cookie Exists as a signed Cookie or not. If so we use the value of the cookie.
    //If not we create a new one.
    if let Some(cookie) = cookie_jar.get_cookie(&config.cookie_name, &config.key) {
        cookie.value().to_owned()
    } else {
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(config.cookie_len)
            .map(char::from)
            .collect()
    }
}
