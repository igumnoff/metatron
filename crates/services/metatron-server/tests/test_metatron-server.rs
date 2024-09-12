use axum_test::TestServer;
use http::header;
use http::HeaderValue;
use http::StatusCode;
use metatron_server::router;
use mime::APPLICATION_PDF;
use serde_json::json;
use std::path::PathBuf;

#[tokio::test]
async fn test_handler() {
    let p = PathBuf::from("../../../");
    let curdir = p.into_os_string();
    let report_template = format!(
        "{}/data/report-template.kdl",
        curdir.clone().into_string().unwrap()
    );
    let report_data = format!("{}/data/report-data.json", curdir.into_string().unwrap());
    let srv = TestServer::new(router());

    let payload = json!({
        "report_template": std::fs::read_to_string(report_template).expect("Failed to read file"),
        "report_data": std::fs::read_to_string(report_data).expect("Failed to read file"),
        "output_format": "pdf"
    });

    let res = srv
        .unwrap()
        .post("/generate")
        .add_header(
            header::CONTENT_TYPE,
            HeaderValue::from_static(APPLICATION_PDF.as_ref()),
        )
        .json(&payload)
        .await;
    assert_eq!(res.status_code(), StatusCode::OK);
    assert_eq!(
        res.headers().get(header::CONTENT_TYPE).unwrap(),
        "application/pdf"
    );
    assert_eq!(res.headers().get(header::CONTENT_ENCODING).unwrap(), "gzip");
    assert_eq!(
        res.headers().get(header::CACHE_CONTROL).unwrap(),
        "public, max-age=31536000"
    );
    assert_eq!(res.as_bytes().len(), 499343);
}
