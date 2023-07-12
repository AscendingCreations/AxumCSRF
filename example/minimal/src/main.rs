use askama::Template;
use axum::{Form, response::IntoResponse, routing::get, Router};
use axum_csrf::{Key, CsrfConfig, CsrfToken};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Template, Deserialize, Serialize)]
#[template(path = "template.html")]
struct Keys {
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
        // `GET /` goes to `root` and Post Goes to check key
        .route("/", get(root).post(check_key))
        .with_state(config);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root(token: CsrfToken) -> impl IntoResponse {
    let keys = Keys {
        authenticity_token: token.authenticity_token().unwrap(),
    };

    // We must return the token so that into_response will run and add it to our response cookies.
    (token, keys).into_response()
}

async fn check_key(token: CsrfToken, Form(payload): Form<Keys>) -> &'static str {
    if token.verify(&payload.authenticity_token).is_err() {
        "Token is invalid"
    } else {
        "Token is Valid lets do stuff!"
    }
}
