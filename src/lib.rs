//!
#![doc = include_str!("../README.md")]
//!

use std::collections::HashMap;
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
        let params_src = data["params"].as_object().ok_or(Common("Missing 'params' in data".to_string()))?;
        let params: HashMap<String, String> = params_src.iter().map(|(k, v)| {
            let value:String = if v.is_number() {
                v.to_string()
            } else {
                v.as_str().unwrap_or("").to_string()
            };
            (k.clone(), value)
        }).collect();
        let mut elements: Vec<Box<dyn Element>> = vec![];
        if let Some(title) = template["title"].as_sequence() {
            for header in title {
                if let Some(header_text) = header["header"].as_str() {
                    let mut resolved_text = header_text.to_string();
                    for (key, value) in &params {
                        resolved_text = resolved_text.replace(&format!("$P{{{}}}", key), &value);
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
                    // Use the `row` section from the YAML template instead of directly using column names
                    if let Some(row_configs) = template["row"].as_sequence() {
                        for row_config in row_configs {
                            if let Some(value_key) = row_config["value"].as_str() {
                                // Determine the field name to extract from the JSON data row based on the template configuration
                                let field_name = value_key.trim_start_matches("$F(").trim_end_matches(")");
                                if let Some(value) = data_row[field_name].as_str() {
                                    let text_element = TextElement::new(value, 8)?; // Default font size for cells
                                    cells.push(TableCellElement::new(&text_element)?);
                                }
                                if let Some(value) = data_row[field_name].as_number() {
                                    let value = value.to_string();
                                    let text_element = TextElement::new(value.as_str(), 8)?; // Default font size for cells
                                    cells.push(TableCellElement::new(&text_element)?);
                                }
                            }
                        }
                    }
                    rows.push(TableRowElement::new(&cells)?);
                }
            }


            if let Some(footer_configs) = template["column_footer"].as_sequence() {
                let mut footer_cells = Vec::new();
                for footer_config in footer_configs {
                    if let Some(value_key) = footer_config["value"].as_str() {
                        // Replace placeholders with actual values
                        let mut resolved_text = value_key.to_string();
                        for (key, value) in &params {
                            resolved_text = resolved_text.replace(&format!("$P{{{}}}", key), &value);
                        }

                        // Handling empty values to maintain table structure
                        if resolved_text.is_empty() {
                            resolved_text = " ".to_string(); // Ensure empty cells are accounted for
                        }

                        let text_element = TextElement::new(&resolved_text, 8)?; // Assuming default font size for footer
                        footer_cells.push(TableCellElement::new(&text_element)?);
                    } else {
                        // If there's no value specified, insert a placeholder or space
                        let text_element = TextElement::new(" ", 8)?; // Ensure the table structure remains consistent
                        footer_cells.push(TableCellElement::new(&text_element)?);
                    }
                }

                // Create a footer row and add it to the rows
                if !footer_cells.is_empty() {
                    let footer_row = TableRowElement::new(&footer_cells)?;
                    // Assuming you have a method to add a row to the table, or you can directly append it if `rows` is accessible here
                    rows.push(footer_row);
                }
            }

            let table_element_with_footer = TableElement::new(&headers, &rows)?;

            elements.push(table_element_with_footer);

        }


        let document = Document::new(&elements)?;
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
    use shiva::core::TransformerTrait;

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
