use hyper::{Server};
use std::net::SocketAddr;
use std::convert::Infallible;
use crate::web::get_all_routes;
use integra::core::router::{ServiceWithRouter};
use std::sync::Arc;
use hyper::service::make_service_fn;
use std::env;

use once_cell::sync::Lazy;
use std::sync::Mutex;
use tera::Tera;

static TERA: Lazy<Mutex<Tera>> = Lazy::new(|| {
    let tera = match Tera::new("src/templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Parsing error(s): {}", e);
            std::process::exit(1);
        }
    };
    Mutex::new(tera)
});

mod routes;
mod web;
mod controllers;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let router = get_all_routes();
    let shared_router = Arc::new(router);

    let make_svc = make_service_fn(move |_conn| {
        let service = ServiceWithRouter { router: shared_router.clone() };
        async move { Ok::<_, Infallible>(service) }
    });

    let ip = env::var("IP").unwrap_or_else(|_| String::from("0.0.0.0"));
    let port = env::var("PORT").unwrap_or_else(|_| String::from("3000"));
    let addr = format!("{}:{}", ip, port).parse::<SocketAddr>().unwrap();
    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}