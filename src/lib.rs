use std::collections::HashMap;
use anyhow::anyhow;
use bytes::Bytes;
use serde_yaml::Value as YValue;
use serde_json::Value as JValue;
use shiva::core::{Document, Element, HeaderElement, TableCellElement, TableElement, TableHeaderElement, TableRowElement, TextElement};

use thiserror::Error;
use crate::ReportError::Common;

pub struct Report;


impl Report {

    pub fn generate(template: &str, data: &str,  _images: &HashMap<String, Bytes>) -> anyhow::Result<Document> {
        let template: YValue = serde_yaml::from_str(template)?;
        let data: JValue = serde_json::from_str(data)?;
        let params = data["params"].as_object().ok_or(Common("Missing 'params' in data".to_string()))?;
        let mut elements: Vec<Box<dyn Element>> = vec![];
        if let Some(title) = template["title"].as_sequence() {
            for header in title {
                if let Some(header_text) = header["header"].as_str() {
                    let mut resolved_text = header_text.to_string();
                    for (key, value) in params {
                        resolved_text = resolved_text.replace(&format!("$P{{{}}}", key), value.as_str().unwrap_or(""));
                    }
                    let header_element = HeaderElement::new(&resolved_text, header["level"].as_u64().unwrap_or(1) as u8)?;
                    elements.push(header_element);
                }
            }
        }



        // Process table, if present in template
        if let Some(columns) = template["column_header"].as_sequence() {
            let mut headers = Vec::new();
            for column in columns {
                if let Some(name) = column["name"].as_str() {
                    let text_element = TextElement::new(name, 8)?; // Assuming default font size for headers
                    let width = column["width"].as_f64().unwrap_or(20.0) as f32; // Default width if not specified
                    let mut header_element = TableHeaderElement::new(&text_element)?;
                    header_element.width = width;
                    headers.push(header_element);
                }
            }

            let mut rows = Vec::new();
            if let Some(data_rows) = data["rows"].as_array() {
                for data_row in data_rows {
                    let mut cells = Vec::new();
                    for column in columns {
                        if let Some(field_name) = column["name"].as_str() {
                            if let Some(value) = data_row[field_name].as_str() {
                                let text_element = TextElement::new(value, 8)?; // Default font size for cells
                                cells.push(TableCellElement::new(&text_element)?);
                            }
                        }
                    }
                    rows.push(TableRowElement::new(&cells)?);
                }
            }

            let table_element = TableElement::new(&headers, &rows)?;
            elements.push(table_element);
        }


        let document = Document::new(&elements)?;
        println!("{:?}", document);
        Ok(document)

    }

}

#[derive(Error, Debug)]
pub enum ReportError {
    #[error("Report error: {0}")]
    Common(String),
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_generate() -> anyhow::Result<()> {
        let template_vec = std::fs::read("data/report-template.yaml")?;
        let template = std::str::from_utf8(&template_vec).unwrap();
        let data_vec = std::fs::read("data/report-data.json")?;
        let data = std::str::from_utf8(&data_vec).unwrap();
        let images = HashMap::new();
        let result = Report::generate(template, data, &images);
        assert!(result.is_ok());
        Ok(())
    }
}
