<h1 align="center">
    Axum_CSRF
</h1>
<div align="center">
    Library to provide a CSRF (Cross-Site Request Forgery) protection layer to axum-based web applications. Axum 0.6 is currently supported.
</div>
<br />
<div align="center">
    <a href="https://crates.io/crates/axum_csrf"><img src="https://img.shields.io/crates/v/axum_csrf?style=plastic" alt="crates.io"></a>
    <a href="https://docs.rs/axum_csrf"><img src="https://docs.rs/axum_csrf/badge.svg" alt="docs.rs"></a>
    <img src="https://img.shields.io/badge/min%20rust-1.60-green.svg" alt="Minimum Rust Version">
</div>

# Help

If you need help with this library please join our [Discord Group](https://discord.gg/gVXNDwpS3Z)

## Install
```toml
# Cargo.toml
[dependencies]
axum_csrf = "0.7.0"
```

# Example

Add it to axum via shared state:
```rust
use askama::Template;
use axum::{Form, response::IntoResponse, routing::get, Router};
use axum_csrf::{CsrfConfig, CsrfToken};
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
    let config = CsrfConfig::default();

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

// root creates the CSRF Token and sends it into the page for return.
async fn root(token: CsrfToken) -> impl IntoResponse {
    let keys = Keys {
        //this Token is a hashed Token. it is returned and the original token is hashed for comparison.
        authenticity_token: token.authenticity_token().unwrap(),
    };

    // We must return the token so that into_response will run and add it to our response cookies.
    (token, keys).into_response()
}

async fn check_key(token: CsrfToken, Form(payload): Form<Keys>) -> &'static str {
    // Verfiy the Hash and return the String message.
    if token.verify(&payload.authenticity_token).is_err() {
        "Token is invalid"
    } else {
        "Token is Valid lets do stuff!"
    }
}
```

The Template File
```html
<!DOCTYPE html>
<html>
    <head>
        <meta charset="UTF-8" />
        <title>Example</title>
    </head>

    <body>
        <form method="post" action="/">
            <input type="hidden" name="authenticity_token" value="{{ authenticity_token }}"/>
            <input id="button" type="submit" value="Submit" tabindex="4" />
        </form>
    </body>
</html>
```

Or use the "layer" feature if you dont want to use state:
```rust
use askama::Template;
use axum::{Form, response::IntoResponse, routing::get, Router};
use axum_csrf::{CsrfConfig, CsrfLayer, CsrfToken };
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
    let config = CsrfConfig::default();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root` and Post Goes to check key
        .route("/", get(root).post(check_key))
        .layer(CsrfLayer::new(config));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// root creates the CSRF Token and sends it into the page for return.
async fn root(token: CsrfToken) -> impl IntoResponse {
    let keys = Keys {
        //this Token is a hashed Token. it is returned and the original token is hashed for comparison.
        authenticity_token: token.authenticity_token().unwrap(),
    };

    // We must return the token so that into_response will run and add it to our response cookies.
    (token, keys).into_response()
}

async fn check_key(token: CsrfToken, Form(payload): Form<Keys>) -> &'static str {
    // Verfiy the Hash and return the String message.
    if token.verify(&payload.authenticity_token).is_err() {
        "Token is invalid"
    } else {
        "Token is Valid lets do stuff!"
    }
}
```

If you already have an encryption key for private cookies, build the CSRF configuration a different way:
```rust
let cookie_key = cookie::Key::generate();
let config = CsrfConfig::default().with_key(Some(cookie_key));

let app = Router::new().with_state(config)
```

# Prevent Post Replay Attacks with CSRF.

If you want to Prevent Post Replay Attacks then you should use a Session Storage method.
you store the hash in the server side session store as well as send it with the form.
when they post the data you would check the hash of the form first and then against the internal session data 2nd.
After the 2nd hash is valid you would then remove the hash from the session.
This prevents replay attacks and ensure no data was manipulated.
If you need a Session database I would suggest using [`axum_session`](https://crates.io/crates/axum_session)

Changes using `axum_session`.
```rust
async fn greet(token: CsrfToken, session: Session<SessionPgPool>) -> impl IntoResponse {
    let authenticity_token = token.authenticity_token();
    session.set("authenticity_token", authenticity_token.clone()).await;

    let keys = Keys {
        authenticity_token,
    }

    //we must return the token so that into_response will run and add it to our response cookies.
    (token, keys).into_response()
}
```

Validate the CSRF Key and Validate for Post Replay attacks
```rust
async fn check_key(token: CsrfToken, session: Session<SessionPgPool>, Form(payload): Form<Keys>,) -> &'static str {
    let authenticity_token: String = session.get("authenticity_token").await.unwrap_or_default();

    if let Err(_) = token.verify(&payload.authenticity_token) {
        "Token is invalid"
    } else if let Err(_) = token.verify(&authenticity_token) {
        "Modification of both Cookie/token OR a replay attack occured"
    } else {
        // we remove it to only allow one post per generated token.
        session.remove("authenticity_token").await;
        "Token is Valid lets do stuff!"
    }
}
```
