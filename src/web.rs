use crate::routes::app;
use integra::core::router::Router;
use integra::routes;
use integra::route_collector;

pub fn get_all_routes() -> Router {
    route_collector!(
        app
    )
}
