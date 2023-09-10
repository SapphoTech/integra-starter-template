use hyper::{Server};
use std::net::SocketAddr;
use std::convert::Infallible;
use crate::routes::get_all_routes;
use integra::core::router::{ServiceWithRouter};
use std::sync::Arc;
use hyper::service::make_service_fn;

mod app;
mod routes;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let router = get_all_routes();
    let shared_router = Arc::new(router);

    let make_svc = make_service_fn(move |_conn| {
        let service = ServiceWithRouter { router: shared_router.clone() };
        async move { Ok::<_, Infallible>(service) }
    });

    let addr = SocketAddr::from(([127, 0, 0, 1], 3011));
    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}