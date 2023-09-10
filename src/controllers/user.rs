use hyper::{Body, Request, Response};

pub struct Controller;

impl Controller {
    pub async fn show(req: Request<Body>) -> Response<Body> { 
        let id = extract_id_from_path(req.uri().path());
        let response_body = format!("Showing user with id: {}", id);
        Response::new(Body::from(response_body))
    }
}

fn extract_id_from_path(path: &str) -> String { 
    path.split("/").last().unwrap_or("").to_string()
}
