use std::sync::{Arc, Mutex};

use axum::{extract::State, routing::post, Json, Router};
use log::info;
use serde::{Deserialize, Serialize};

use crate::database::Database;

/// Starts a metric server.
#[tokio::main]
pub async fn start(database: Arc<Mutex<Database>>) {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();

    let state = AppState { database };

    let app = Router::new()
        .route("/get-metrics", post(get_metrics))
        .with_state(state);

    info!("listening on port 4000");

    axum::serve(listener, app).await.unwrap();
}

/// Shared application state.
#[derive(Clone, Debug)]
struct AppState {
    database: Arc<Mutex<Database>>,
}

/// Input for the get_metrics operation.
#[derive(Deserialize, Debug)]
struct GetMetricsInput {
    /// The name of the metric to get datapoints for.
    name: String,
}

/// Output for the get_metrics operation.
#[derive(Serialize, Debug)]
struct GetMetricsOutput {
    /// Metrics datapoints. Each item in the list is a time-value pair. The time
    /// represents a UNIX timestamp in seconds.
    metrics: Vec<(u64, f64)>,
}

/// Gets metric datapoints.
async fn get_metrics(
    State(state): State<AppState>,
    Json(input): Json<GetMetricsInput>,
) -> Json<GetMetricsOutput> {
    let database = state.database.lock().unwrap();
    let metrics = database.get_metrics(input.name);
    let output = GetMetricsOutput { metrics };
    Json(output)
}
