mod utils;
use utils::to_u8arr;

use std::collections::HashMap;
use std::io;
use mime;
use std::io::Read;
use std::sync::Arc;
use axum::{routing::{get, post}, http::StatusCode, Json, Router, response, body::HttpBody};
use axum::body::{Body, BodyDataStream};
use axum::http::header;
use axum::response::{IntoResponse, Response};
use bytes::Bytes;
use metatron::Report;
use serde::{Deserialize, Serialize, Serializer};
use shiva::core::{Document, DocumentType, TransformerTrait};
use tokio::io::ReadBuf;
use tokio::runtime;
use tokio_util::io::ReaderStream;



#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/generate", post(handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


async fn handler(Json(payload): Json<CreateDocument>) -> impl IntoResponse{
    let images = HashMap::new();
    let report = Report::generate(
        &payload.report_template,
        &payload.report_data,
        &images
    );

    let file = match report {
        Ok(file) => {file},
        Err(err) => return Err((StatusCode::BAD_REQUEST, format!("File is corrupted: {}", err)))
    };

    let document: &'static Bytes = &to_u8arr(&file, payload.output_format);
    // https://habr.com/ru/articles/499108/
    let document_iter = document.into_iter();
    let document_as_bytes: Vec<_> = document_iter.collect();
    let as_vector = document_as_bytes.as_slice();
    let stream = ReaderStream::new(as_vector);
    let body = Body::from_stream(stream);

    let headers = response::AppendHeaders([
        (header::CONTENT_TYPE, "text/toml; charset=utf-8"),
        (
            header::CONTENT_DISPOSITION,
            "attachment; filename=\"file.txt\"",
        ),
    ]);

    let response = Response::builder()
        .header("Content-Encoding", "gzip")
        .header("Content-Type", mime::PDF.as_str())
        .header("Cache-Control", "public, max-age=31536000")
        .status(StatusCode::OK)
        .body(body)
        .unwrap();

    return Ok(response);
}


#[derive(Deserialize)]
struct CreateDocument {
    pub report_template: String,
    pub report_data: String,
    pub output_format: DocumentType,
}


#[cfg(test)]
mod tests{
    use super::*;
}

