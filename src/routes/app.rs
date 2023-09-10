use crate::controllers::dashboard::Controller as DashboardController;
use crate::controllers::page::Controller as PageController;
use crate::controllers::user::Controller as UserController;
use integra::core::router::Route;

pub fn routes() -> Vec<Route> {
    vec![
        Route::get("/", DashboardController::index),
        Route::get("/hello", PageController::greet),
        Route::get("/user/{id}", UserController::show)
    ]
}
