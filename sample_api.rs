use hyper::{Body, Method, Request, Response, Server, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;

#[derive(Debug, Deserialize)]
struct RequestPayload {
    name: String,
    age: u32,
}

#[derive(Debug, Serialize)]
struct ResponseBody {
    message: String,
}

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/api") => {
            let whole_body = hyper::body::to_bytes(req.into_body()).await.unwrap();
            let payload: RequestPayload = serde_json::from_slice(&whole_body).unwrap();

            // Process the payload
            let message = format!("Hello, {}! You are {} years old.", payload.name, payload.age);
            let response_body = ResponseBody { message };
            let response_json = serde_json::to_string(&response_body).unwrap();

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(Body::from(response_json))
                .unwrap())
        }
        _ => {
            let response_body = "Not Found";
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from(response_body))
                .unwrap())
        }
    }
}

#[tokio::main]
async fn main() {
    let addr = ([127, 0, 0, 1], 3000).into();
    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(handle_request))
    });
    let server = Server::bind(&addr).serve(make_svc);
    println!("Server listening on http://{}", addr);
    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
