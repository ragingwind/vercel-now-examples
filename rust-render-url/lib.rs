pub mod index;

extern crate http;

use http::{Request, Response};

pub fn handler(request: Request<()>) -> http::Result<Response<String>> {
    index::handler(request)
}