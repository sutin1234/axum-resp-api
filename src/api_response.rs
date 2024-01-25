use axum::{
    body::Body,
    http::{HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use bytes::Bytes;
use hyper::header::{ACCESS_CONTROL_EXPOSE_HEADERS, CONTENT_DISPOSITION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};

// here we show a type that implements Serialize + Send
#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub message: String,
}

pub enum ApiResponse {
    OK,
    Created,
    JsonData(Message),
    DataBytes(Bytes),
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> Response {
        match self {
            Self::OK => (StatusCode::OK).into_response(),
            Self::Created => (StatusCode::CREATED).into_response(),
            Self::JsonData(data) => (StatusCode::OK, Json(data)).into_response(),
            Self::DataBytes(data) => {
                let mut res = Body::from(data).into_response();
                res.headers_mut().insert(
                    CONTENT_TYPE,
                    HeaderValue::from_static("application/oct-stream"),
                );
                res.headers_mut().insert(
                    ACCESS_CONTROL_EXPOSE_HEADERS,
                    HeaderValue::from_static("content-disposition"),
                );
                res.headers_mut().insert(
                    CONTENT_DISPOSITION,
                    HeaderValue::from_static(
                        "attachment; filename=\"my_pdf.pdf\"; name=\"my_pdf\"",
                    ),
                );
                return res;
            }
        }
    }
}
