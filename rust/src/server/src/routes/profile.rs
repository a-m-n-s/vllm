use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;

use crate::error::ApiError;
use crate::state::AppState;
use crate::utils::utility_call_error;

/// Start engine profiling. Mirrors Python's `POST /start_profile`, which
/// forwards to the engine `profile(is_start=True)` utility call.
pub async fn start_profile(State(state): State<Arc<AppState>>) -> Result<StatusCode, ApiError> {
    state
        .engine_core_client()
        .profile(true, None)
        .await
        .map_err(|error| utility_call_error("profile", error))?;

    Ok(StatusCode::OK)
}

/// Stop engine profiling. Mirrors Python's `POST /stop_profile`, which forwards
/// to the engine `profile(is_start=False)` utility call.
pub async fn stop_profile(State(state): State<Arc<AppState>>) -> Result<StatusCode, ApiError> {
    state
        .engine_core_client()
        .profile(false, None)
        .await
        .map_err(|error| utility_call_error("profile", error))?;

    Ok(StatusCode::OK)
}
