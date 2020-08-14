use crate::analyzer::HeapDump;
mod routes;

pub fn create_server(address: String, heapdump: HeapDump) {
    println!("Listening for requests at http://{}", address);
    gotham::start(address, routes::heapviewRouter(heapdump))
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use gotham::hyper::StatusCode;
//     use gotham::test::TestServer;

//     #[test]
//     fn receive_hello_world_response() {
//         let test_server = TestServer::new(|| Ok(say_hello)).unwrap();
//         let response = test_server
//             .client()
//             .get("http://localhost")
//             .perform()
//             .unwrap();

//         assert_eq!(response.status(), StatusCode::OK);

//         let body = response.read_body().unwrap();
//         assert_eq!(&body[..], b"Hello World!");
//     }
// }
