//!
#![doc = include_str!("../README.md")]
//!

use bytes::Bytes;
use serde_json::Value as JValue;
use serde_yaml::Value as YValue;
use shiva::core::Element::{Header, Paragraph, Table, Text};
use shiva::core::{Document, Element, TableCell, TableHeader, TableRow};
use std::collections::HashMap;

use crate::ReportError::Common;
use thiserror::Error;

pub struct Report;

impl Report {
    pub fn generate(
        template: &str,
        data: &str,
        _images: &HashMap<String, Bytes>,
    ) -> anyhow::Result<Document> {
        let template: YValue = serde_yaml::from_str(template)?;
        let data: JValue = serde_json::from_str(data)?;
        let params_src = data["params"]
            .as_object()
            .ok_or(Common("Missing 'params' in data".to_string()))?;
        let params: HashMap<String, String> = params_src
            .iter()
            .map(|(k, v)| {
                let value: String = if v.is_number() {
                    v.to_string()
                } else {
                    v.as_str().unwrap_or("").to_string()
                };
                (k.clone(), value)
            })
            .collect();
        let mut elements: Vec<Element> = vec![];
        let mut page_header: Vec<Element> = vec![];
        let mut page_footer: Vec<Element> = vec![];
        if let Some(title) = template["title"].as_sequence() {
            for header in title {
                if let Some(header_text) = header["header"].as_str() {
                    let mut resolved_text = header_text.to_string();
                    for (key, value) in &params {
                        resolved_text = resolved_text.replace(&format!("$P{{{}}}", key), &value);
                    }
                    let header_element = Header {
                        text: resolved_text,
                        level: header["level"].as_u64().unwrap_or(1) as u8,
                    };
                    elements.push(header_element);
                }
            }
        }

        if let Some(columns) = template["column_header"].as_sequence() {
            let mut headers = Vec::new();
            for column in columns {
                if let Some(name) = column["name"].as_str() {
                    let text_element = Text {
                        text: name.to_string(),
                        size: 8,
                    };
                    let width = column["width"].as_f64().unwrap_or(20.0) as f32;
                    let header_element = TableHeader {
                        element: Box::new(text_element),
                        width: width,
                    };
                    headers.push(header_element);
                }
            }

            let mut rows = Vec::new();
            if let Some(data_rows) = data["rows"].as_array() {
                for data_row in data_rows {
                    let mut cells = Vec::new();
                    if let Some(row_configs) = template["row"].as_sequence() {
                        for row_config in row_configs {
                            if let Some(value_key) = row_config["value"].as_str() {
                                let field_name =
                                    value_key.trim_start_matches("$F(").trim_end_matches(")");
                                if let Some(value) = data_row[field_name].as_str() {
                                    let text_element = Text {
                                        text: value.to_string(),
                                        size: 8,
                                    };
                                    cells.push(TableCell {
                                        element: Box::new(text_element),
                                    });
                                }
                                if let Some(value) = data_row[field_name].as_number() {
                                    let value = value.to_string();
                                    let text_element = Text {
                                        text: value,
                                        size: 8,
                                    }; // Default font size for cells
                                    cells.push(TableCell {
                                        element: Box::from(text_element),
                                    });
                                }
                            }
                        }
                    }
                    rows.push(TableRow { cells });
                }
            }

            if let Some(footer_configs) = template["column_footer"].as_sequence() {
                let mut footer_cells = Vec::new();
                for footer_config in footer_configs {
                    if let Some(value_key) = footer_config["value"].as_str() {
                        let mut resolved_text = value_key.to_string();
                        for (key, value) in &params {
                            resolved_text =
                                resolved_text.replace(&format!("$P{{{}}}", key), &value);
                        }
                        if resolved_text.is_empty() {
                            resolved_text = " ".to_string();
                        }
                        let text_element = Text {
                            text: resolved_text,
                            size: 8,
                        };
                        footer_cells.push(TableCell {
                            element: Box::new(text_element),
                        });
                    } else {
                        let text_element = Text {
                            text: " ".to_string(),
                            size: 8,
                        };
                        footer_cells.push(TableCell {
                            element: Box::from(text_element),
                        });
                    }
                }
                if !footer_cells.is_empty() {
                    let footer_row = TableRow {
                        cells: footer_cells,
                    };
                    rows.push(footer_row);
                }
            }

            let table_element_with_footer = Table { headers, rows };

            elements.push(table_element_with_footer);
        }

        if let Some(headers) = template["page_header"].as_sequence() {
            for header in headers {
                if let Some(header_text) = header["text"].as_str() {
                    let text_size = header["size"].as_u64().unwrap_or(7) as u8; // Default size if not specified
                    let text_element = Text {
                        text: header_text.to_string(),
                        size: text_size,
                    };
                    page_header.push(text_element);
                }
            }
        }

        if let Some(footers) = template["page_footer"].as_sequence() {
            for footer in footers {
                if let Some(footer_text) = footer["text"].as_str() {
                    let text_size = footer["size"].as_u64().unwrap_or(7) as u8; // Default size if not specified
                    let text_element = Text {
                        text: footer_text.to_string(),
                        size: text_size,
                    };
                    page_footer.push(text_element);
                }
            }
        }

        if let Some(summary) = template["summary"].as_sequence() {
            for paragraph_config in summary {
                if let Some(paragraph_items) = paragraph_config["paragraph"].as_sequence() {
                    let mut paragraph_elements: Vec<Element> = vec![];
                    for text_item in paragraph_items {
                        if let Some(text_value) = text_item["text"].as_str() {
                            let mut resolved_text = text_value.to_string();
                            for (key, value) in &params {
                                resolved_text =
                                    resolved_text.replace(&format!("$P{{{}}}", key), value);
                            }
                            let text_size = text_item["size"].as_u64().unwrap_or(10) as u8; // Default size if not specified
                            let text_element = Text {
                                text: resolved_text,
                                size: text_size,
                            };
                            paragraph_elements.push(text_element);
                        }
                    }
                    if !paragraph_elements.is_empty() {
                        let paragraph_element = Paragraph {
                            elements: paragraph_elements,
                        };
                        elements.push(paragraph_element);
                    }
                }
            }
        }

        let mut document = Document::new(elements);
        document.page_header = page_header;
        document.page_footer = page_footer;
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
    use shiva::core::TransformerTrait;
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
        let doc = result?;
        println!("{:?}", doc);
        println!("=========================");
        let result = shiva::markdown::Transformer::generate(&doc)?;
        let result_str = std::str::from_utf8(&result.0)?;
        println!("{}", result_str);
        Ok(())
    }
}
