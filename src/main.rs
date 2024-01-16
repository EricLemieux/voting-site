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

#[derive(Template)]
#[template(path = "button.html")]
pub struct Button {
    click_count: u32,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
struct VoteOption {
    /// The internal name that is used for matching.
    id: String,

    /// The name that is displayed to the user.
    display: String,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
enum Direction {
    Up,
    Down,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
struct VoteInput {
    index: usize,
    direction: Direction,
    state: Vec<VoteOption>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
struct VoteResult {
    state: Vec<VoteOption>,
}

async fn handle_vote_up() -> () {
    todo!("call change_ranking");
}
async fn handle_vote_down() -> () {
    todo!("call change_ranking");
}

fn change_ranking(input: &VoteInput) -> VoteResult {
    println!(
        "vote-registered | {:?} | {:?} | {:?}",
        input.state, input.index, input.direction
    );

    let swap_index: usize = match input.direction {
        Direction::Up => match input.index {
            0 => 0,
            _ => input.index - 1,
        },
        Direction::Down => match input.index {
            _ if input.index == input.state.len() - 1 => input.index,
            _ => input.index + 1,
        },
    };

    if input.index == swap_index {
        println!("Input index and the swap index are at the same position {}", swap_index);
        //return VoteResult { state: input.state };
    }

    let mut new_state = input.state.clone();
    new_state.swap(input.index, swap_index);

    return VoteResult { state: new_state };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn change_ranking_test() {
        let params = vec![
            (
                VoteInput {
                    state: vec![
                        create_vopt("a"),
                        create_vopt("b"),
                        create_vopt("c"),
                        create_vopt("d"),
                    ],
                    index: 2,
                    direction: Direction::Up,
                },
                VoteResult {
                    state: vec![
                        create_vopt("a"),
                        create_vopt("c"),
                        create_vopt("b"),
                        create_vopt("d"),
                    ],
                },
            ),
            (
                VoteInput {
                    state: vec![
                        create_vopt("a"),
                        create_vopt("b"),
                        create_vopt("c"),
                        create_vopt("d"),
                    ],
                    index: 0,
                    direction: Direction::Up,
                },
                VoteResult {
                    state: vec![
                        create_vopt("a"),
                        create_vopt("b"),
                        create_vopt("c"),
                        create_vopt("d"),
                    ],
                },
            ),
            (
                VoteInput {
                    state: vec![
                        create_vopt("a"),
                        create_vopt("b"),
                        create_vopt("c"),
                        create_vopt("d"),
                    ],
                    index: 2,
                    direction: Direction::Down,
                },
                VoteResult {
                    state: vec![
                        create_vopt("a"),
                        create_vopt("b"),
                        create_vopt("d"),
                        create_vopt("c"),
                    ],
                },
            ),
            (
                VoteInput {
                    state: vec![
                        create_vopt("a"),
                        create_vopt("b"),
                        create_vopt("c"),
                        create_vopt("d"),
                    ],
                    index: 3,
                    direction: Direction::Down,
                },
                VoteResult {
                    state: vec![
                        create_vopt("a"),
                        create_vopt("b"),
                        create_vopt("c"),
                        create_vopt("d"),
                    ],
                },
            ),
        ];

        for param in params {
            let actual = change_ranking(&param.0);

            assert_eq!(param.1, actual);
        }
    }

    fn create_vopt(name: &str) -> VoteOption {
        return VoteOption {
            id: name.to_string(),
            display: name.to_string(),
        };
    }
}

// things that need to be added
//
// routes
//  - /vote-up
//  - /vote-down
//  - /submit
//
//
// internal state
// Needs to be stored somewhere, should it be an anonim something or other? Or could the whole
// state be passed around in the query params?
//
// I think the qury params method would be more interesting.
//
// state the needs to be tracked
//  - current-index
//
//        ---that might be it if we know the index and th---
//        relying on the server to have the full context, which won't work if trying to keep the logic be
//        differnt
//
//---
//
// The logic might still be interesting. recursive tree traversal.in onder to build the state of
// the world. So  that might be work working on my own anyway.
