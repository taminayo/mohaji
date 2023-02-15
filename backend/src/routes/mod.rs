mod hello;

use axum::{routing::get, Router};
use hello::hello;

pub fn create_routes() -> Router<> {
    Router::new().route("/", get(hello))
}
