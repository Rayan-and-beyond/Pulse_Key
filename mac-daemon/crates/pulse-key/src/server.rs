use crate::engine::{Event, Snapshot};
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde_json::json;
use std::path::PathBuf;
use tokio::sync::{broadcast, mpsc, watch};

#[derive(Clone)]
pub struct AppState {
    pub snapshot: watch::Receiver<Snapshot>,
    pub cmd: mpsc::Sender<Event>,
    pub extras: broadcast::Sender<String>,
    pub vault: PathBuf,
}

pub fn state_json(s: &Snapshot) -> serde_json::Value {
    json!({ "type": "state", "state": s.state, "reason": s.reason, "bpm": s.bpm })
}

pub fn spawn_broadcaster(
    mut snap_rx: watch::Receiver<Snapshot>,
    extras: broadcast::Sender<String>,
) {
    tokio::spawn(async move {
        while snap_rx.changed().await.is_ok() {
            let s = snap_rx.borrow().clone();
            let msg = serde_json::to_string(&state_json(&s)).unwrap();
            let _ = extras.send(msg);
        }
    });
}

pub fn router(app: AppState) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/state", get(get_state))
        .route("/request-unlock", post(request_unlock))
        .route("/vault", get(vault))
        .route("/events", get(ws_handler))
        .with_state(app)
}

async fn index() -> Html<&'static str> {
    Html(include_str!("../web/index.html"))
}

async fn get_state(State(app): State<AppState>) -> Json<serde_json::Value> {
    Json(state_json(&app.snapshot.borrow()))
}

async fn request_unlock(State(app): State<AppState>) -> Response {
    let state = app.snapshot.borrow().state.clone();
    if state == "unlocked" || state == "armed" || state == "measuring" {
        return (
            StatusCode::CONFLICT,
            Json(json!({ "error": format!("already {state}") })),
        )
            .into_response();
    }
    let _ = app.cmd.send(Event::RequestUnlock).await;
    (StatusCode::ACCEPTED, Json(json!({ "ok": true }))).into_response()
}

async fn vault(State(app): State<AppState>) -> Response {
    if app.snapshot.borrow().state != "unlocked" {
        return (StatusCode::FORBIDDEN, Json(json!({ "error": "locked" }))).into_response();
    }
    match std::fs::read_to_string(&app.vault) {
        Ok(content) => content.into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

async fn ws_handler(ws: WebSocketUpgrade, State(app): State<AppState>) -> Response {
    ws.on_upgrade(move |socket| ws_loop(socket, app))
}

async fn ws_loop(mut socket: WebSocket, app: AppState) {
    let mut rx = app.extras.subscribe();
    let initial = serde_json::to_string(&state_json(&app.snapshot.borrow())).unwrap();
    if socket.send(Message::Text(initial.into())).await.is_err() {
        return;
    }
    loop {
        match rx.recv().await {
            Ok(msg) => {
                if socket.send(Message::Text(msg.into())).await.is_err() {
                    break;
                }
            }
            Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
            Err(tokio::sync::broadcast::error::RecvError::Closed) => break,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    fn test_app(state: &str, vault: PathBuf, cmd: mpsc::Sender<Event>) -> (Router, watch::Sender<Snapshot>) {
        let (tx, rx) = watch::channel(Snapshot { state: state.into(), reason: None, bpm: None });
        let (extras, _) = broadcast::channel(8);
        (router(AppState { snapshot: rx, cmd, extras, vault }), tx)
    }

    async fn body_text(resp: axum::response::Response) -> String {
        String::from_utf8(resp.into_body().collect().await.unwrap().to_bytes().to_vec()).unwrap()
    }

    #[tokio::test]
    async fn index_serves_page() {
        let (cmd, _rx) = mpsc::channel(4);
        let (app, _tx) = test_app("locked", PathBuf::from("vault.txt"), cmd);
        let resp = app.oneshot(Request::builder().uri("/").body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        assert!(body_text(resp).await.contains("Pulse Key"));
    }

    #[tokio::test]
    async fn vault_is_forbidden_when_locked() {
        let (cmd, _rx) = mpsc::channel(4);
        let (app, _tx) = test_app("locked", PathBuf::from("vault.txt"), cmd);
        let resp = app.oneshot(Request::builder().uri("/vault").body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(resp.status(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn vault_is_served_when_unlocked() {
        let path = std::env::temp_dir().join(format!("pulse-key-vault-test-{}.txt", std::process::id()));
        std::fs::write(&path, "secret payload").unwrap();
        let (cmd, _rx) = mpsc::channel(4);
        let (app, _tx) = test_app("unlocked", path.clone(), cmd);
        let resp = app.oneshot(Request::builder().uri("/vault").body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(body_text(resp).await, "secret payload");
        let _ = std::fs::remove_file(path);
    }

    #[tokio::test]
    async fn request_unlock_reaches_engine_channel() {
        let (cmd, mut rx) = mpsc::channel(4);
        let (app, _tx) = test_app("locked", PathBuf::from("vault.txt"), cmd);
        let req = Request::builder()
            .method("POST")
            .uri("/request-unlock")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::ACCEPTED);
        assert!(matches!(rx.recv().await, Some(Event::RequestUnlock)));
    }

    #[tokio::test]
    async fn request_unlock_conflicts_when_unlocked() {
        let (cmd, _rx) = mpsc::channel(4);
        let (app, _tx) = test_app("unlocked", PathBuf::from("vault.txt"), cmd);
        let req = Request::builder()
            .method("POST")
            .uri("/request-unlock")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::CONFLICT);
    }
}
