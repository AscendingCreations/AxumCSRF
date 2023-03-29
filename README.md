<h1 align="center">
    Axum_CSRF
</h1>
<div align="center">
    Library to provide a CSRF (Cross-Site Request Forgery) protection layer to axum-based web applications.
</div>
<br />
<div align="center">
    <a href="https://crates.io/crates/axum_csrf"><img src="https://img.shields.io/crates/v/axum_csrf?style=plastic" alt="crates.io"></a>
    <a href="https://docs.rs/axum_csrf"><img src="https://docs.rs/axum_csrf/badge.svg" alt="docs.rs"></a>
    <img src="https://img.shields.io/badge/min%20rust-1.60-green.svg" alt="Minimum Rust Version">
</div>

# Help

If you need help with this library please join our [Discord Group](https://discord.gg/xKkm7UhM36)

## Install
```toml
# Cargo.toml
[dependencies]
axum_csrf = "0.6.0"
```

# Example

Add it to axum via shared state:
```rust
#[tokio::main]
async fn main() {

    // Build our application with some routes
    let app = Router::new()
        .route("/greet", get(greet))
        .route("/check_key", post(check_key))
        .with_state(CsrfConfig::default());

    // Serve the application at http://localhost:3000/
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

If you already have an encryption key for private cookies, build the CSRF configuration a different way:
```rust
let cookie_key = cookie::Key::generate();
let config = CsrfConfig::default().with_key(Some(cookie_key));

let app = Router::new().with_state(config)
```

Add the attribute to your form view template context struct:
```rust
#[derive(Template, Deserialize, Serialize)]
#[template(path = "hello.html")]
struct Keys {
    authenticity_token: String,
    // Your attributes...
}
```

Retrieve the CSRF token, extracted as axum request handler parameter. Insert its authenticity token hash into the form view's template context:
```rust
async fn greet(token: CsrfToken) -> impl IntoResponse {
    let keys = Keys {
        authenticity_token: token.authenticity_token(),
    }

    // We must return the token so that into_response will run and add it to our response cookies.
    (token, HtmlTemplate(keys))
}
```

Insert the authenticity token into the HTML template as hidden form field:
```html
<form method="post" action="/check_key">
    <input type="hidden" name="authenticity_token" value="{{ authenticity_token }}"/>
    <!-- your fields -->
</form>
```


Validate the CSRF key upon receiving the `POST`ed form data:
```rust
async fn check_key(token: CsrfToken, Form(payload): Form<Keys>,) -> &'static str {
    if let Err(_) = token.verify(&payload.authenticity_token) {
        "Token is invalid"
    } else {
        "Token is Valid lets do stuff!"
    }
}
```

# Prevent Post Replay Attacks with CSRF.

If you want to Prevent Post Replay Attacks then you should use a Session Storage method.
you store the hash in the server side session store as well as send it with the form.
when they post the data you would check the hash of the form first and then against the internal session data 2nd.
After the 2nd hash is valid you would then remove the hash from the session.
This prevents replay attacks and ensure no data was manipulated.
If you need a Session database I would suggest using [`axum_database_sessions`](https://crates.io/crates/axum_database_sessions)

Changes using `axum_database_sessions`.
```rust
async fn greet(token: CsrfToken, sessions: AxumSession) -> impl IntoResponse {
    let authenticity_token = token.authenticity_token();
    session.set("authenticity_token", authenticity_token.clone()).await;

    let keys = Keys {
        authenticity_token,
    }

    //we must return the token so that into_response will run and add it to our response cookies.
    (token, HtmlTemplate(keys))
}
```

Validate the CSRF Key and Validate for Post Replay attacks
```rust
async fn check_key(token: CsrfToken, sessions: AxumSession, Form(payload): Form<Keys>,) -> &'static str {
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
