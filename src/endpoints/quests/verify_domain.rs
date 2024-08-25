use crate::{
    common::verify_has_root_or_braavos_domain::verify_has_root_or_braavos_domain,
    models::{AppState, VerifyQuery},
};
use axum::{
    extract::{Query, State},
    response::IntoResponse,
};
use axum_auto_routes::route;
use std::sync::Arc;

#[route(get, "/quests/verify_has_domain")]
pub async fn handler(
    State(state): State<Arc<AppState>>,
    Query(query): Query<VerifyNewQuery>,
) -> impl IntoResponse {
    verify_has_root_or_braavos_domain(state, &query.addr, 82).await
}
