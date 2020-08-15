use std::sync::Arc;

use gotham::helpers::http::response::create_response;
use gotham::hyper::{Body, Response, StatusCode};
use gotham::middleware::state::StateMiddleware;
use gotham::pipeline::single::single_pipeline;
use gotham::pipeline::single_middleware;
use gotham::router::builder::*;
use gotham::router::Router;
use gotham::state::{FromState, State};
use mime;
use serde_json::{self, json};

use crate::analyzer::Stats;

/// Used to share analysis results across execution threads
#[derive(Debug, Clone, StateData)]
struct SharedHeapdump {
    inner: Arc<Stats>,
}

pub fn handler(state: State) -> (State, Response<Body>) {
    let stats: &Stats = &*SharedHeapdump::borrow_from(&state).inner;

    let mut response = match serde_json::to_string(&stats) {
        Ok(value) => create_response(&state, StatusCode::OK, mime::APPLICATION_JSON, value),
        Err(e) => {
            eprintln!("Error when serializing stats: {:?}", e);
            create_response(
                &state,
                StatusCode::INTERNAL_SERVER_ERROR,
                mime::APPLICATION_JSON,
                json!({"message": "Failed to parse stats"}).to_string(),
            )
        }
    };

    {
        let headers = response.headers_mut();
        headers.insert("Access-Control-Allow-Origin", "*".parse().unwrap());
    };

    (state, response)
}

pub fn heapview_router(stats: Stats) -> Router {
    let shared = SharedHeapdump {
        inner: Arc::new(stats),
    };
    let middleware = StateMiddleware::new(shared);

    let pipeline = single_middleware(middleware);
    let (chain, pipelines) = single_pipeline(pipeline);

    build_router(chain, pipelines, |route| {
        route.scope("/analysis", |route| route.get("/").to(handler))
    })
}
