use http::{Request, Response, StatusCode};

pub fn handler(request: Request<()>) -> http::Result<Response<String>> {
    let uri = request.uri();
    let response = Response::builder()
        .status(StatusCode::OK)
        .body(format!("{}", uri))
        .expect("failed");

    Ok(response)
}

#[test]
fn test_request_handler() {
    let mut req = Request::builder();
    req.uri("https://now.sh/")
       .header("User-Agent", "now-agent/1.0");
    let res = handler(req.body(()).unwrap());
    
    assert!(res.is_ok());
    assert_eq!(res.unwrap().body(), "https://now.sh/");
}