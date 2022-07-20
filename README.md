# Axum_CSRF

Library to Provide a CSRF (Cross-Site Request Forgery) protection layer.

[![https://crates.io/crates/axum_csrf](https://img.shields.io/badge/crates.io-v0.5.0-blue)](https://crates.io/crates/axum_csrf)
[![Docs](https://docs.rs/axum_csrf/badge.svg)](https://docs.rs/axum_csrf)

# Example

Add it to Axums via layer.
```rust
#[tokio::main]
async fn main() {
    let config = //load your config here.
    let poll = init_pool(&config).unwrap();

    let session_config = SqlxSessionConfig::default()
        .with_database("test")
        .with_table_name("test_table");

    // build our application with some routes
    let app = Router::new()
        .route("/greet", get(greet))
        .route("/check_key", post(check_key))
        .layer(CsrfLayer::new(CsrfConfig::default()));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

If you already have an encryption key for private cookies, build the layer a different way:
```rust
let cookie_key = cookie::Key::generate(); // or from()/derive_from()
let config = CsrfConfig::default().with_key(Some(cookie_key));
let csrf_layer = CsrfLayer::new(config);

let app = Router::new()
    // ...
    .layer(csrf_layer);
```

Get the Hash for the Form to insert into the html for return.
```rust
async fn greet(token: CsrfToken) -> impl IntoResponse {
    let keys = Keys {
        authenticity_token: token.authenticity_token(),
    }

    //we must return the token so that into_response will run and add it to our response cookies.
    (token, HtmlTemplate(keys))
}
```

Insert it in the html form
```html
<form method="post" action="/check_key">
    <input type="hidden" name="authenticity_token" value="{{ authenticity_token }}"/>
    <!-- your fields -->
</form>
```

Add the Attribute to your form return structs
```rust
#[derive(Template, Deserialize, Serialize)]
#[template(path = "hello.html")]
struct Keys {
    authenticity_token: String,
    // your attributes
}
```

Validate the CSRF Key
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
This prevents replay attacks and ensure no data was munipulated.
If you need a Session database I would suggest using [Axum_database_sessions](https://crates.io/crates/axum_database_sessions)

Changes using axum_database_sessions.
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

# Help

If you need help with this library please go to our [Discord Group](https://discord.gg/xKkm7UhM36)
