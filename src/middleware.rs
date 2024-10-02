use axum::{
    middleware::{self, Next},
    response::Response,
    http::Request,
};
use std::time::Instant;

pub async fn log_requests(req: Request<axum::body::Body>, next: Next) -> Response {
    let start_time = Instant::now();
    let response = next.run(req).await; // Теперь корректно
    let duration = start_time.elapsed();

    println!("Request took: {:?}", duration);
    response
}
