use std::{fs::File, io::Read};

use axum::http::Response;
use bytes::Bytes;

use crate::{api_response::ApiResponse, error_response::ApiError};

pub async fn pdf_handler() -> Result<ApiResponse, ApiError> {
    let path = "public/pdf/my_pdf.pdf";
    let mut file = File::open(path).unwrap();
    let mut pdf_bytes: Vec<_> = Vec::new();
    file.read_to_end(&mut pdf_bytes).unwrap();

    // Set headers
    let response = Response::new(Bytes::from(pdf_bytes));

    let bytes = response.to_owned().into_body();
    Ok(ApiResponse::DataBytes(bytes))
}
