use metatron::Report;
use shiva::core::TransformerTrait;
use std::collections::HashMap;
use tracing::info;

fn template_data() -> Result<(String, String), anyhow::Error> {
    let template = std::fs::read_to_string("../../../data/report-template.kdl")?;
    let data = std::fs::read_to_string("../../../data/report-data.json")?;
    Ok((template, data))
}

#[test]
fn test_generate() -> anyhow::Result<()> {
    let (template, data) = template_data()?;
    let images = HashMap::new();
    let result = Report::generate(&template, &data, &images, "pdf");
    assert!(result.is_ok());
    std::fs::write("../../../data/report_generate.pdf", result.unwrap())?;
    Ok(())
}

#[test]
fn test_generate_by_shiva() -> anyhow::Result<()> {
    let (template, data) = template_data()?;
    let images = HashMap::new();
    let result = Report::to_document(&template, &data, &images);
    info!("{:?}", result);
    assert!(result.is_ok());
    let doc = result?;
    info!("{:?}", doc);
    info!("=========================");
    let result = shiva::pdf::Transformer::generate(&doc)?;
    std::fs::write("../../../data/report.pdf", result)?;
    let result = shiva::html::Transformer::generate(&doc)?;
    std::fs::write("../../../data/report.html", result)?;

    Ok(())
}

#[test]
fn test_to_u8arr() -> anyhow::Result<()> {
    let (template, data) = template_data()?;
    let images = HashMap::new();
    let result = Report::generate(&template, &data, &images, "pdf");
    let doc = result?;
    assert_eq!(doc.len(), 499343);
    Ok(())
}
