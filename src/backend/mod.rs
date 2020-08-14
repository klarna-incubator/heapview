mod routes;

pub fn create_server(address: String) {
    println!("Listening for requests at http://{}", address);
    gotham::start(address, routes::heapview_router())
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
