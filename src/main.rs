use axum::{routing::get, routing::post, Router};

use askama::Template;

static mut BUTTON_CLICK_COUNT: u32 = 0;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root_handler)).route("/button", post(button_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9090").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct Root {}

async fn root_handler() -> Root {
    println!("request to root");
    return Root {
    }
}

#[derive(Template)]
#[template(path = "button.html")]
pub struct Button {
    click_count: u32
}

async fn button_handler() -> Button {
    unsafe {
    println!("button clicked");
    BUTTON_CLICK_COUNT += 1;
    return Button {
    click_count: BUTTON_CLICK_COUNT
    };
    }
}
