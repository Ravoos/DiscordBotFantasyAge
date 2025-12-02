use tiny_http::{Server, Response}

pub fn start_health_server(){
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".into());
    let addr = format!("0.0.0.0:{}", port);
    let server = Server::http(addr).except("Failed to start health server"); 

    std::thread::spawn(move || {
        for request in server.incoming_requests(){
            let response = Response::from_string("OK");
            let _ = request.respond(response);
        }
    });
}