use std::collections::HashMap;
use std::io;
use std::io::Read;
use std::sync::Arc;

use axum::{body::HttpBody, http::StatusCode, Json, response, Router, routing::{get, post}};
use axum::body::{Body, BodyDataStream};
use axum::http::header;
use axum::response::{IntoResponse, Response};
use bytes::buf::Reader;
use bytes::Bytes;
use mime;
use serde::{Deserialize, Serialize, Serializer};
use shiva::core::{Document, DocumentType, TransformerTrait};
use tokio::io::ReadBuf;
use tokio::runtime;
use tokio_util::io::ReaderStream;

use metatron::Report;
use utils::to_u8arr;

mod utils;

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
        Err(err) => return Err((StatusCode::NOT_ACCEPTABLE, format!("File is corrupted: {}", err)))
    };

    let document: Bytes = to_u8arr(&file, payload.output_format);
    let body = Body::from(document);

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
    use axum_test::TestServer;
    use http::{HeaderValue, Request};
    use http::StatusCode;
    use serde_json::json;
    use tokio;
    use std::env;

    use super::*;

    #[tokio::test]
    async fn test_handler(){
        let curdir = env::current_dir().unwrap().into_os_string();
        let report_template = format!("{}/data/report-template.kdl", curdir.clone().into_string().unwrap());
        let report_data = format!("{}/data/report-data.json", curdir.into_string().unwrap());
        let app = Router::new().route("/generate", post(handler));
        let mut srv = TestServer::new(app);

        let payload = json!({
            "report_template": std::fs::read_to_string(report_template).unwrap(),
            "report_data": std::fs::read_to_string(report_data).unwrap(),
            "output_format": "Pdf"
        });

        let res = srv.unwrap().post("/generate")
            .add_header(header::CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .json(&payload).await;
        assert_eq!(res.status_code(), StatusCode::OK);
    }
}

