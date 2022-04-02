# Axum_CSRF

Library to Provide a CSRF (Cross-Site Request Forgery) protection layer.

[![https://crates.io/crates/axum_csrf](https://img.shields.io/badge/crates.io-v0.3.0-blue)](https://crates.io/crates/axum_csrf)
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

Get the Hash for the Form to insert into the html for return.
```rust
async fn greet(token: CsrfToken) -> impl IntoResponse {
    let keys = Keys {
        authenticity_token: token.authenticity_token(),
    }

    HtmlTemplate(keys)
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