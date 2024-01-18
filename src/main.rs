use std::{collections::HashMap, sync::Arc, str::FromStr};

use axum::{extract::Query, routing::get, routing::post, Extension, Form, Router, http::Response};
use serde::Deserialize;

use askama::Template;
use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
use uuid::Uuid;

const DB_URL: &str = "sqlite:voting-site.db";

#[derive(Clone, Debug)]
struct Context {
    db: SqlitePool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let connection_options = SqliteConnectOptions::from_str(DB_URL)?.create_if_missing(true);
    let pool = SqlitePool::connect_with(connection_options).await?;
    sqlx::migrate!().run(&pool).await?;

    let context = Arc::new(Context { db: pool });
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/save-vote", post(save_vote))
        .layer(Extension(context));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9090").await.unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct Root {
    vote_options: Vec<VoteOption>,
}

async fn root_handler(Query(params): Query<HashMap<String, String>>) -> Root {
    println!("request to root");
    println!("{:?}", params);
    // TODO: Get params, that should store the options
    let options = vec![
        VoteOption {
            id: String::from("one"),
            display: String::from("One"),
        },
        VoteOption {
            id: String::from("two"),
            display: String::from("Two"),
        },
    ];
    return Root {
        vote_options: options,
    };
}

#[derive(Template)]
#[template(path = "vote-saved.html")]
pub struct SaveVoteResponse {}

#[axum::debug_handler]
async fn save_vote(
    context: Extension<Arc<Context>>,
    Form(payload): Form<HashMap<String, String>>,
) -> anyhow::Result<SaveVoteResponse, Response<String>> {
    println!("{:?}", payload);

    let mut ranking: Vec<String> = vec![];

    for opt in payload.into_iter() {
        ranking.push(opt.0);
    }

    let ranking_str = ranking.join(",");

    // TODO: This should then save into the db
    match sqlx::query(r#"insert into votes (id, ranking) values ($1, $2)"#)
        .bind(Uuid::new_v4())
        .bind(ranking_str)
        .execute(&context.db)
        .await
    {
        Ok(_res) => {
            return Ok(SaveVoteResponse {});
        }
        Err(_) => {
            return Err(Response::new("something".to_string()));
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
struct VoteOption {
    /// The internal name that is used for matching.
    id: String,

    /// The name that is displayed to the user.
    display: String,
}
