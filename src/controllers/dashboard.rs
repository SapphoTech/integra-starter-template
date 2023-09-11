use hyper::{Body, Request, Response}; 
use std::collections::HashMap;  
use crate::TERA;
use integra::render;
use serde_json::json;

pub struct Controller;

impl Controller {
    pub async fn show(_req: Request<Body>) -> Response<Body> {
        let data = json!({
            "title": "My Site",
            "page_title": "Dashboard"
        });

        render!("dashboard.html", &data)
    }
 

    pub async fn index(_req: Request<Body>) -> Response<Body> { 
        Response::new(Body::from("Dashboard"))
    }
}

