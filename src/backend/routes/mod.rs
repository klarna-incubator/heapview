use gotham::router::builder::*;
use gotham::router::Router;
mod analysis;

pub fn heapviewRouter() -> Router {
    build_simple_router(|route| {
        route.scope("/analysis", |route| {
            route.get("/").to(analysis::index_handler)
        })
    })
}
