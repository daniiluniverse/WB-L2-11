use std::net::SocketAddr;
use axum::{routing::{post, Router}, ServiceExt};
use std::sync::Arc;
use tokio::sync::Mutex;
use log::LevelFilter;
use env_logger;
use tokio::net::windows::named_pipe::PipeEnd::Server;

mod models;
mod handlers;
mod middleware;
mod utils;

#[tokio::main]
async fn main() {
    let app = Router::new(); // Ваш маршрутизатор

    // Укажите адрес и порт
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    // Запуск сервера
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
