use serde_json::json;

use crate::api_response::{ApiResponse, Message};
use crate::error_response::ApiError;

pub async fn index_handler() -> Result<ApiResponse, ApiError> {
    let json = json!({
        "message": "Hello Axum Resp API"
    });
    let json_to_struct: Message = serde_json::from_value(json).unwrap();
    return Ok(ApiResponse::JsonData(json_to_struct));
}
