use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;

// The handler function
async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Hello, World!")))
}

#[tokio::main]
async fn main() {
    // Render requires 0.0.0.0 for the address and an exposed port
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Server running on http://{}", addr);

    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(hello_world)) });
    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
