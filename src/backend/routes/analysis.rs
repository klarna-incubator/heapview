// extern crate serde_derive;
use gotham::handler::IntoResponse;
use gotham::helpers::http::response::create_response;
use gotham::hyper::{Body, Response, StatusCode};
use gotham::router::builder::*;
use gotham::router::Router;
use gotham::state::State;
use serde::{Deserialize, Serialize};

/// An Analysis
#[derive(Serialize, Deserialize)]
pub struct Analysis {
    name: String,
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

pub fn index_handler(state: State) -> (State, Analysis) {
    let analysis = Analysis {
        name: "analysis".to_string(),
    };

    (state, analysis)
}
