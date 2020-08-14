use std::fs;

use gotham::router::builder::*;
use gotham::router::Router;
use gotham::hyper::{Body, Response, StatusCode};
use gotham::state::State;
use gotham::helpers::http::response::create_response;
use mime;

mod analysis;


pub fn handler(state: State) -> (State, Response<Body>) {

    // file
    let data = fs::read_to_string("/tmp/stats.json").expect("NOPE");

    let mut response = create_response(
        &state,
        StatusCode::OK,
        mime::TEXT_PLAIN,
        data,
    );

    {
        let headers = response.headers_mut();
        headers.insert("Access-Control-Allow-Origin", "*".parse().unwrap());
        headers.insert("Content-Type", "application/json".parse().unwrap());
    };

    (state, response)
}

pub fn heapview_router() -> Router {
    // let heapdumpMiddleware = StateMiddleware::with(heapdump);
    // let (chain, pipeline) = single_pipeline(new_pipeline().add(heapdumpMiddleware).build());
    // create the counter to share across handlers
    // let req_heapdump = RequestHeapdump::new(heapdump);

    // // create our state middleware to share the counter
    // let middleware = StateMiddleware::new(req_heapdump);

    // // create a middleware pipeline from our middleware
    // let pipeline = single_middleware(middleware);

    // // construct a basic chain from our pipeline
    // let (chain, pipelines) = single_pipeline(pipeline);

    build_simple_router(|route| {
        route.scope("/analysis", |route| {
            route.get("/").to(handler)
        })
    })
}
