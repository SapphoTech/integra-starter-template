use hyper::{Body, Request, Response};

pub struct Controller;

impl Controller {
    pub async fn greet(_req: Request<Body>) -> Response<Body> { 
        Response::new(Body::from("Hello from PageController!"))
    }
}
