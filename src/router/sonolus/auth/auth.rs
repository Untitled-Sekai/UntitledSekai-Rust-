use axum::{
    response::Json,
    http::StatusCode,
    extract::rejection::JsonRejection,
};
use crate::model::auth::{ServerAuthenticateRequest, ServerAuthenticateResponse};
use crate::router::sonolus::utils::generate::generate_session_info;

pub async fn auth_handler(
    payload: Result<Json<ServerAuthenticateRequest>, JsonRejection>,
) -> Result<Json<ServerAuthenticateResponse>, StatusCode> {
    let Json(payload) = match payload {
        Ok(json) => json,
        Err(rejection) => {
            tracing::error!("JSON parsing error: {:?}", rejection);
            return Err(StatusCode::UNPROCESSABLE_ENTITY);
        }
    };

    // リクエストタイプの検証を追加
    if payload.request_type != "authenticateServer" {
        tracing::warn!("Invalid request type: {}", payload.request_type);
        return Err(StatusCode::UNPROCESSABLE_ENTITY);
    }

    let session_info = generate_session_info(Some(30));
    
    tracing::info!(
        "Authentication successful for user: {} ({})", 
        payload.user_profile.name, 
        payload.user_profile.id
    );

    let response = ServerAuthenticateResponse {
        session: session_info.session,
        expiration: session_info.expiration * 1000, // ミリ秒に変換
    };

    Ok(Json(response))
}