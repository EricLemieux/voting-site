use std::collections::HashMap;

use axum::{routing::get, routing::post, Router, extract::Query, Form};
use serde::Deserialize;

use askama::Template;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root_handler))
        .route("/save-vote", post(save_vote));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9090").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct Root {
    vote_options: Vec<VoteOption>,
}

async fn root_handler(Query(params): Query<HashMap<String, String>>)  -> Root {
    println!("request to root");
    println!("{:?}", params);
    // TODO: Get params, that should store the options
    let options = vec!(
        VoteOption {
            id: String::from("one"),
            display: String::from("One"),
        },
        VoteOption {
            id: String::from("two"),
            display: String::from("Two"),
        },
    );
    return Root {
        vote_options: options,
    };
}

#[derive(Template)]
#[template(path = "vote-saved.html")]
pub struct SaveVoteResponse {}

async fn save_vote(Form(payload): Form<HashMap<String, String>>) -> SaveVoteResponse {
    println!("{:?}", payload);
    
    let mut ranking: Vec<String> = vec!();

    for opt in payload.into_iter() {
        ranking.push(opt.0); 
    }

    // TODO: This should then save into the db
    println!("{:?}", ranking);

    return SaveVoteResponse{};
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
struct VoteOption {
    /// The internal name that is used for matching.
    id: String,

    /// The name that is displayed to the user.
    display: String,
}

