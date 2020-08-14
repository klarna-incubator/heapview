
use crate::analyzer::Stats;
use gotham::handler::IntoResponse;
use gotham::helpers::http::response::create_response;
use gotham::hyper::{Body, Response, StatusCode};
use gotham::state::State;
use serde::Serialize;



/// An Analysis
#[derive(Serialize)]
pub struct Analysis {
    stats: Stats,
}

impl IntoResponse for Analysis {
    fn into_response(self, state: &State) -> Response<Body> {
        create_response(
            state,
            StatusCode::OK,
            mime::APPLICATION_JSON,
            serde_json::to_string(&self).expect("serialized product"),
        )
    }
}

// pub fn index_handler(state: State) -> (State, Analysis) {

//     // let analysis = read_analysis_from_file()
//     // let req_heapdump = RequestHeapdump::borrow_from(&state);

//     // let heapdump = req_heapdump.inner.lock().unwrap();

//     (state, analysis)
// }
