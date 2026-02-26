use axum::extract::{Query, Request};
use axum::response::IntoResponse;
use axum::{Json, routing::get};
use serde_json::json;
use tokio::net::TcpListener;

use axum::Router;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub(crate) struct Server;

impl Server {
    pub(crate) async fn new(host: &Option<String>, port: &Option<String>) -> crate::Result<Self> {
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                    format!(
                        "{}=debug,tower_http=debug,axum::rejection=trace",
                        env!("CARGO_CRATE_NAME")
                    )
                    .into()
                }),
            )
            .with(tracing_subscriber::fmt::layer())
            .init();

        let app = Router::new()
            .route("/", get(index::get).post(index::post))
            .route("/load-test", get(load_test::get))
            .route("/ping", get(ping::get))
            .layer(tower_http::trace::TraceLayer::new_for_http());

        let host = host.as_deref().unwrap_or("0.0.0.0");
        let port = port.as_deref().unwrap_or("3500");

        let listener = TcpListener::bind(format!("{host}:{port}")).await.unwrap();
        tracing::info!("listening on {}", listener.local_addr().unwrap());
        axum::serve(listener, app).await.unwrap();

        Ok(Self)
    }
}

mod index {

    use super::*;

    pub(super) async fn get() -> impl IntoResponse {
        Json(json!({ "received": true }))
    }

    #[derive(serde::Deserialize)]
    pub(crate) struct StatusQuery {
        status: Option<u16>,
    }

    pub(super) async fn post(Query(query): Query<StatusQuery>, req: Request) -> impl IntoResponse {
        let status_code = query
            .status
            .and_then(|s| axum::http::StatusCode::from_u16(s).ok())
            .unwrap_or(axum::http::StatusCode::OK);

        let headers: serde_json::Map<String, serde_json::Value> = req
            .headers()
            .iter()
            .map(|(name, value)| {
                (
                    name.to_string(),
                    serde_json::Value::String(value.to_str().unwrap_or("<binary>").to_string()),
                )
            })
            .collect();

        let body_bytes = axum::body::to_bytes(req.into_body(), usize::MAX)
            .await
            .unwrap_or_default();

        let payload: serde_json::Value = serde_json::from_slice(&body_bytes)
            .unwrap_or(json!({"_raw": String::from_utf8_lossy(&body_bytes).to_string()}));

        tracing::debug!("{:#}", json!({ "headers": headers, "payload": payload }));

        (status_code, Json(payload))
    }
}

mod load_test {
    use super::*;

    const LOREM_IPSUM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. \
        Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim \
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. \
        Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat \
        nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia \
        deserunt mollit anim id est laborum.";

    #[derive(serde::Deserialize)]
    pub(super) struct LoadTestQuery {
        count: Option<usize>,
    }

    #[derive(serde::Serialize)]
    struct LoadTestResponse<'a> {
        lorem_ipsum: &'a str,
        count: usize,
        items: Vec<LoadTestItem<'a>>,
    }

    #[derive(serde::Serialize)]
    struct LoadTestItem<'a> {
        index: usize,
        value: String,
        raw: &'a str,
    }

    pub(super) async fn get(Query(query): Query<LoadTestQuery>) -> impl IntoResponse {
        let count = query.count.unwrap_or(1);

        let items: Vec<LoadTestItem> = (0..count)
            .map(|i| LoadTestItem {
                index: i,
                value: format!("item_{i}"),
                raw: LOREM_IPSUM,
            })
            .collect();

        Json(LoadTestResponse {
            count,
            lorem_ipsum: LOREM_IPSUM,
            items,
        })
    }
}

mod ping {
    use super::*;

    pub(super) async fn get() -> impl IntoResponse {
        Json(json!({ "data": "pong" }))
    }
}
