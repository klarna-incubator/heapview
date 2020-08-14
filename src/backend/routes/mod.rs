use crate::analyzer::HeapDump;
use gotham::router::builder::*;
use gotham::router::Router;
mod analysis;
use gotham::middleware::state::StateMiddleware;
use gotham::pipeline::single_middleware;
use gotham::pipeline::{new_pipeline, single::single_pipeline};
use gotham::state::{FromState, State};
use std::sync::{Arc, Mutex};

// fn router(repo: Repo) -> Router {
//     // Add the diesel middleware to a new pipeline
//     let (chain, pipeline) =
//         single_pipeline(new_pipeline().add(DieselMiddleware::new(repo)).build());

//     // Build the router
//     build_router(chain, pipeline, |route| {
//         route.get("/").to(get_products_handler);
//         route.post("/").to(create_product_handler);
//     })
// }

#[derive(Clone, StateData)]
pub struct RequestHeapdump {
    inner: Arc<Mutex<HeapDump>>,
}

pub impl RequestHeapdump {
    /// Creates a new request counter, setting the base state to `0`.
    fn new(heapdump: HeapDump) -> Self {
        Self {
            inner: Arc::new(Mutex::new(heapdump)),
        }
    }
}

pub fn heapviewRouter(heapdump: HeapDump) -> Router {
    // let heapdumpMiddleware = StateMiddleware::with(heapdump);
    // let (chain, pipeline) = single_pipeline(new_pipeline().add(heapdumpMiddleware).build());
    // create the counter to share across handlers
    let req_heapdump = RequestHeapdump::new(heapdump);

    // create our state middleware to share the counter
    let middleware = StateMiddleware::new(req_heapdump);

    // create a middleware pipeline from our middleware
    let pipeline = single_middleware(middleware);

    // construct a basic chain from our pipeline
    let (chain, pipelines) = single_pipeline(pipeline);

    build_router(chain, pipelines, |route| {
        route.scope("/analysis", |route| {
            route.get("/").to(analysis::index_handler)
        })
    })
}
