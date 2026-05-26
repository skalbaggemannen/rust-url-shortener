use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Json, Router,
};
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

type Db = Arc<Mutex<HashMap<String, String>>>;

#[derive(Clone)]
struct AppState {
    db: Db,
}

#[derive(Deserialize)]
struct ShortenRequest {
    url: String,
}

#[derive(Serialize)]
struct ShortenResponse {
    code: String,
    short_url: String,
    original_url: String,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        db: Arc::new(Mutex::new(HashMap::new())),
    };

    let app = Router::new()
        .route("/", get(home))
        .route("/shorten", post(shorten_url))
        .route("/{code}", get(redirect_to_url))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("Server running at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn home() -> &'static str {
    "Rust URL Shortener is running. POST /shorten with { \"url\": \"https://example.com\" }"
}

async fn shorten_url(
    State(state): State<AppState>,
    Json(payload): Json<ShortenRequest>,
) -> impl IntoResponse {
    if !payload.url.starts_with("http://") && !payload.url.starts_with("https://") {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "URL must start with http:// or https://"
            })),
        )
            .into_response();
    }

    let code = nanoid!(6);

    {
        let mut db = state.db.lock().unwrap();
        db.insert(code.clone(), payload.url.clone());
    }

    let response = ShortenResponse {
        code: code.clone(),
        short_url: format!("http://localhost:3000/{}", code),
        original_url: payload.url,
    };

    (StatusCode::CREATED, Json(response)).into_response()
}

async fn redirect_to_url(
    State(state): State<AppState>,
    Path(code): Path<String>,
) -> impl IntoResponse {
    let db = state.db.lock().unwrap();

    match db.get(&code) {
        Some(url) => Redirect::temporary(url).into_response(),
        None => (
            StatusCode::NOT_FOUND,
            format!("No URL found for code: {}", code),
        )
            .into_response(),
    }
}
