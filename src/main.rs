use axum::{routing::get, Router};

use askama::Template;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/foo", get(foo_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9090").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

#[derive(Template)]
#[template(path = "foo.html")]
pub struct Foo {}

async fn foo_handler() -> Foo {
    return Foo {};
}
