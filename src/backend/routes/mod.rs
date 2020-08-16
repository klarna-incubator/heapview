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

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;

    use serde_json::{from_str, Value};

    use crate::analyzer::{NodeType, Stats};
    use gotham::hyper::StatusCode;
    use gotham::test::TestServer;

    #[test]
    fn test_analysis() {
        let mut categories = HashMap::new();
        categories.insert(NodeType::Array, 111);
        categories.insert(NodeType::Number, 222);
        categories.insert(NodeType::String, 333);

        let stats = Stats {
            total: 666,
            categories,
        };
        let test_server = TestServer::new(heapview_router(stats)).unwrap();

        let response = test_server
            .client()
            .get("http://localhost:3000/analysis")
            .perform()
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.read_body().unwrap();
        let b: Value = from_str(std::str::from_utf8(&body[..]).unwrap()).unwrap();

        let expected = json!(
            {
                "total": 666,
                "categories": {"array": 111, "number": 222, "string": 333}
            }
        );

        assert_eq!(b, expected);
    }
}
