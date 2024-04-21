use std::collections::HashMap;
use std::io::Bytes;
use axum::{routing::{get, post}, http::StatusCode, Json, Router, response};
use metatron::Report;
use serde::{Deserialize, Serialize};
use shiva::core::{Document, TransformerTrait};
use tokio::runtime;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/generate", post(generate_document));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


fn handle_document_generation(
    response_object: &mut CreateDocumentResponse,
    document: anyhow::Result<(bytes::Bytes, HashMap<String, bytes::Bytes>)>
){
    if !document.is_ok() {
        response_object.status.push_str("server error");
    }
    let document = document?;
    response_object.report_file = document.0;
}


async fn generate_document(
    Json(payload): Json<CreateDocument>,
) -> (StatusCode, Json<CreateDocumentResponse>) {
    let images = HashMap::new();
    let report = Report::generate(
        &payload.report_template,
        &payload.report_data,
        &images
    );

    let mut response_object = CreateDocumentResponse{
        report_file: bytes::Bytes::new(),
        status: String::new()
    };

    let result = report.is_ok();
    if !result {
        response_object.status.push_str("document is possibly corrupted");
        return (StatusCode::BAD_REQUEST, Json(response_object));
    }
    let generated_report = report?;

    match payload.output_format {
        DocumentFormats::Pdf => {
            let result = shiva::pdf::Transformer::generate(&generated_report);
            handle_document_generation(& mut response_object, result);
            if response_object.status == "server error" {
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(response_object));
            }
        }
        DocumentFormats::Text => {
            let result = shiva::text::Transformer::generate(&generated_report);
            handle_document_generation(& mut response_object, result);
            if response_object.status == "server error" {
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(response_object));
            }
        }
        DocumentFormats::Html => {
            let result = shiva::html::Transformer::generate(&generated_report);
            handle_document_generation(& mut response_object, result);
            if response_object.status == "server error" {
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(response_object));
            }
        }
        DocumentFormats::Markdown => {
            let result = shiva::markdown::Transformer::generate(&generated_report);
            handle_document_generation(& mut response_object, result);
            if response_object.status == "server error" {
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(response_object));
            }
        }
    }
    (StatusCode::CREATED, Json(response_object))
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

#[derive(Serialize)]
struct CreateDocumentResponse {
    pub report_file: bytes::Bytes,
    pub status: String,
}

