use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use std::convert::Infallible;

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    // Handle the incoming request and generate a response
    let response_body = "Hello, World!";
    let response = Response::new(Body::from(response_body));

    Ok(response)
}

#[tokio::main]
async fn main() {
    // Define the address on which the server will listen
    let addr = ([127, 0, 0, 1], 3000).into();

    // Create a make_service_fn closure to create a service for each incoming connection
    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(handle_request))
    });

    // Create a new Server
    let server = Server::bind(&addr).serve(make_svc);

    // Print a message to indicate that the server has started
    println!("Server listening on http://{}", addr);

    // Run the server indefinitely
    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
