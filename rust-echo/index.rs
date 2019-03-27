use http::{Request, Response, StatusCode};
use std::collections::HashMap;
use url::Url;

pub fn handler(request: Request<()>) -> http::Result<Response<String>> {
    let url_str = request.uri().to_string();
    let url = Url::parse(&url_str).unwrap();
    let hash_query: HashMap<_, _> = url.query_pairs().to_owned().collect();

    match hash_query.get("echo") {
        Some(ref echo) => {
            let response = Response::builder()
                .status(StatusCode::OK)
                .body(format!("{}", echo))
                .expect("failed");
            Ok(response)
        }

        _ => Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body("echo parameter is missing".to_string()),
    }
}

#[test]
fn test_request_handler() {
    let req = Request::get("https://now.sh?echo=hello").body(()).unwrap();
    let res = handler(req);
    
    assert!(res.is_ok());
    assert_eq!(res.unwrap().body().to_string(), "hello");
}