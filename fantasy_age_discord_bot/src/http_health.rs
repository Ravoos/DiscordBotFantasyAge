use hyper::{Request, Response, Error};
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use http_body_util::Full;
use bytes::Bytes;
use tokio::net::TcpListener;

pub async fn start_health_server() {
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".into());
    let addr = format!("0.0.0.0:{}", port);

    let listener = TcpListener::bind(&addr)
        .await
        .expect("Failed to bind health server");

    println!("Health server running on {}", addr);

    tokio::spawn(async move {
        loop {
            let (stream, _) = match listener.accept().await {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Health accept error: {}", e);
                    continue;
                }
            };

            tokio::spawn(async move {
                let io = TokioIo::new(stream);

                let service = service_fn(|_req: Request<_>| async {
                    let body = Full::new(Bytes::from("OK"));
                    Ok::<_, Error>(Response::new(body))
                });

                if let Err(e) =
                    hyper::server::conn::http1::Builder::new()
                        .serve_connection(io, service)
                        .await
                {
                    eprintln!("Health server connection error: {}", e);
                }
            });
        }
    });
}
