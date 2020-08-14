use gotham::router::builder::*;
use gotham::router::Router;
mod analysis;


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
            route.get("/").to_file("/tmp/stats.json") //.to(analysis::index_handler)
        })
    })
}
