use std::convert::Infallible;
use std::net::SocketAddr;
use futures::TryStreamExt as _;
use hyper::body::HttpBody;
use hyper::{Method, Body, Request, Response, Server, StatusCode};
use hyper::service::{make_service_fn, service_fn};

// a simple service w/ some routing
async fn echo(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            *response.body_mut() = Body::from("Try POSTing data to /echo");
        },
        (&Method::POST, "/echo") => {
            *response.body_mut() = req.into_body();
        },
        (&Method::POST, "/echo/uppercase") => {
            let mapping = req
                .into_body()
                .map_ok(|chunk| {
                    chunk.iter()
                    .map(|byte| byte.to_ascii_uppercase())
                    .collect::<Vec<u8>>()
                });

            *response.body_mut() = Body::wrap_stream(mapping);
        },
        (&Method::POST, "/echo/reverse") => {
            let upper = req.body().size_hint().upper().unwrap_or(u64::MAX);

            if upper > 1024 * 64 {
                *response.body_mut() = Body::from("body too large");
                *response.status_mut() = StatusCode::PAYLOAD_TOO_LARGE;
                return Ok(response);
            }

            let full_body = hyper::body::to_bytes(req.into_body());
            
            match full_body.await {
                Ok(full_btyes) => {
                    let reversed = full_btyes
                        .iter()
                        .rev()
                        .cloned()
                        .collect::<Vec<u8>>();
                    *response.body_mut() = reversed.into();
                },
                Err(e) => {
                    eprintln!("parsing error {}", e);
                    *response.body_mut() = Body::from("error parsing body");
                    *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                    return Ok(response);
                }
            }
        },
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        },
    };
    
    Ok(response)
}

#[tokio::main]
async fn main() {
    // bind to 127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    
    // creates a "service" to handle connections
    // just tied to our "hello_world" service
    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(echo))
    });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
