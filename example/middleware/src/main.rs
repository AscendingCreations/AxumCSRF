use askama::Template;
use axum::{
    body::Body,
    extract::Request,
    http::{Method, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    routing::{get, post},
    Form, Router,
};
use axum_csrf::{CsrfConfig, CsrfLayer, CsrfToken, Key};
use http_body_util::BodyExt;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

#[derive(Template, Deserialize, Serialize)]
#[template(path = "template.html")]
pub struct Keys {
    authenticity_token: String,
    // Your attributes...
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();
    let cookie_key = Key::generate();
    let config = CsrfConfig::default().with_key(Some(cookie_key));

    // build our application with a route
    let app = Router::new()
        .route("/", post(check_key))
        .layer(axum::middleware::from_fn(auth_middleware))
        // `GET /` goes to `root` and Post Goes to check key
        .route("/", get(root))
        .layer(CsrfLayer::new(config));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root(token: CsrfToken) -> impl IntoResponse {
    let keys = Keys {
        authenticity_token: token.authenticity_token().unwrap(),
    };

    // We must return the token so that into_response will run and add it to our response cookies.
    (token, keys).into_response()
}

/// Can only be done with the feature layer enabled
pub async fn auth_middleware(
    token: CsrfToken,
    method: Method,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    if method == Method::POST {
        let (parts, body) = request.into_parts();

        let bytes = body
            .collect()
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .to_bytes()
            .to_vec();

        let value = serde_urlencoded::from_bytes(&bytes)
            .map_err(|_| -> StatusCode { StatusCode::INTERNAL_SERVER_ERROR })?;
        let payload: Form<Keys> = Form(value);
        if token.verify(&payload.authenticity_token).is_err() {
            return Err(StatusCode::UNAUTHORIZED);
        }

        request = Request::from_parts(parts, Body::from(bytes));
    }

    Ok(next.run(request).await)
}

async fn check_key() -> &'static str {
    "Token is Valid lets do stuff!"
}
