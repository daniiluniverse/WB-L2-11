use std::sync::Arc;
use axum::{extract::State, Json, response::IntoResponse};
use tokio::sync::Mutex as TokioMutex; // Переименование токио мьютекса
use crate::models::{ApiResponse, Event};
use crate::utils::parse_event_params;

pub async fn create_event(State(events): State<Arc<TokioMutex<Vec<Event>>>>, body: String) -> impl IntoResponse {
    match parse_event_params(&body) {
        Ok(event) => {
            let mut events = events.lock().await; // Ожидаем блокировку мьютекса
            events.push(event.clone()); // Добавляем событие в вектор
            Json(ApiResponse { result: Some("Event created".to_string()), error: None }) // Успешный ответ
        }
        Err(e) => {
            // Обработка ошибки с невалидными параметрами
            Json(ApiResponse { result: None, error: Some(e) }) // Возврат ошибки
        }
    }
}
