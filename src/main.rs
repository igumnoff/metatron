use std::collections::HashMap;
use std::io;
use std::io::Read;
use axum::{routing::{get, post}, http::StatusCode, Json, Router, response, body::BodyDataStream};
use axum::http::header;
use axum::response::IntoResponse;
use bytes::Bytes;
use metatron::Report;
use serde::{Deserialize, Serialize, Serializer};
use shiva::core::{Document, TransformerTrait};
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

async fn handler(Json(payload): Json<CreateDocument>,) -> impl IntoResponse {
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
    let stream = ReaderStream::new(file);
    let body = BodyDataStream::new(stream);

    let headers = response::AppendHeaders([
        (header::CONTENT_TYPE, "text/toml; charset=utf-8"),
        (
            header::CONTENT_DISPOSITION,
            "attachment; filename=\"Cargo.toml\"",
        ),
    ]);

    Ok((headers, body))
}

#[derive(Deserialize, Serialize)]
enum DocumentFormats{
    Pdf,
    Text,
    Html,
    Markdown
}

#[derive(Deserialize)]
struct CreateDocument {
    pub report_template: String,
    pub report_data: String,
    pub output_format: DocumentFormats,
}


#[cfg(test)]
mod tests{
    use super::*;
}

