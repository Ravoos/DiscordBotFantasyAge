use hyper::{Request, Response};
use hyper::body::Body;
use hyper::server::Server;
use hyper::service::{make_service_fn, service_fn};
use std::convert::Infallible;
use std::net::SocketAddr;
use tokio::task;

pub fn start_health_server() {
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".into());
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().unwrap();

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(|_req: Request<Body>| async {
            Ok::<_, Infallible>(Response::new(Body::from("OK")))
        }))
    });

    task::spawn(async move {
        let server = Server::bind(&addr).serve(make_svc);
        println!("Health server running on {}", addr);
        if let Err(e) = server.await {
            eprintln!("Health server error: {}", e);
        }
    });
}

