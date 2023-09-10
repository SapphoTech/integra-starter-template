use hyper::{Body, Request, Response};

pub struct Controller;

impl Controller {
    pub async fn index(_req: Request<Body>) -> Response<Body> { 
        Response::new(Body::from("Dashboard"))
    }
}
