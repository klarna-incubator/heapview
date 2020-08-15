use crate::analyzer::Stats;

mod routes;

pub fn create_server(address: String, stats: Stats) {
    println!("Listening for requests at http://{}", address);
    gotham::start(address, routes::heapview_router(stats))
}
