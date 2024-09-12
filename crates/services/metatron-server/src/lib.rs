use axum::{routing::post, Router};
use mime::APPLICATION_PDF;
use std::collections::HashMap;

use axum::body::Body;
use axum::response::{IntoResponse, Response};
use axum::{http::StatusCode, Json};
use metatron::Report;
use serde::Deserialize;

async fn handler(Json(payload): Json<CreateDocument>) -> impl IntoResponse {
    let images = HashMap::new();
    let report = Report::generate(
        &payload.report_template,
        &payload.report_data,
        &images,
        &payload.output_format,
    );

    let Ok(report) = report else {
        return Err((
            StatusCode::NOT_ACCEPTABLE,
            format!("File is corrupted: {}", report.err().unwrap()),
        ));
    };

    let body = Body::from(report);

    let response = Response::builder()
        .header("Content-Encoding", "gzip")
        .header("Content-Type", APPLICATION_PDF.as_ref())
        .header("Cache-Control", "public, max-age=31536000")
        .status(StatusCode::OK)
        .body(body);

    match response {
        Ok(response) => Ok(response),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to create response: {}", e),
        )),
    }
}

pub fn router() -> Router {
    Router::new().route("/generate", post(handler))
}

#[derive(Deserialize)]
struct CreateDocument {
    pub report_template: String,
    pub report_data: String,
    pub output_format: String,
}
