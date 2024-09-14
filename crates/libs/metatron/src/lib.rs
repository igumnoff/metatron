//!
// #![doc = include_str!("../../../../README.md")]
//!
mod error;

use bytes::Bytes;
use error::ReportError::{self, *};
use kdl::KdlDocument;
use serde_json::Value as JValue;
use shiva::core::Element::{Header, Paragraph, Table, Text};
use shiva::core::{
    Document, DocumentType, Element, ImageAlignment, ImageData, ImageDimension, ImageType,
    TableCell, TableHeader, TableRow,
};
use std::collections::HashMap;
use std::str::FromStr;
use tracing::debug;

#[derive(Debug)]
pub struct Report;

impl Report {
    pub fn generate(
        template: &str,
        data: &str,
        images: &HashMap<String, Bytes>,
        document_type: &str,
    ) -> Result<Bytes, ReportError> {
        let document_type = DocumentType::from_str(document_type)
            .map_err(|_| ReportError::InvalidDocumentType(document_type.to_string()))?;

        let document = Self::to_document(template, data, images)?;

        let result = document.generate(document_type);

        match result {
            Ok(bytes) => Ok(bytes),
            Err(e) => Err(ReportError::Common(e.to_string())),
        }
    }

    pub fn to_pdf(
        template: &str,
        data: &str,
        images: &HashMap<String, Bytes>,
    ) -> Result<Bytes, ReportError> {
        let result = Self::generate(template, data, images, "pdf");
        match result {
            Ok(bytes) => Ok(bytes),
            Err(e) => Err(ReportError::Common(e.to_string())),
        }
    }

    pub fn to_html(
        template: &str,
        data: &str,
        images: &HashMap<String, Bytes>,
    ) -> Result<Bytes, ReportError> {
        let result = Self::generate(template, data, images, "html");
        match result {
            Ok(bytes) => Ok(bytes),
            Err(e) => Err(ReportError::Common(e.to_string())),
        }
    }

    pub fn to_text(
        template: &str,
        data: &str,
        images: &HashMap<String, Bytes>,
    ) -> Result<Bytes, ReportError> {
        let result = Self::generate(template, data, images, "text");
        match result {
            Ok(bytes) => Ok(bytes),
            Err(e) => Err(ReportError::Common(e.to_string())),
        }
    }

    pub fn to_document(
        template_str: &str,
        data: &str,
        _images: &HashMap<String, Bytes>,
    ) -> Result<Document, ReportError> {
        let doc: KdlDocument = template_str.parse()?;
        let template_elements = doc
            .get("template")
            .ok_or(Common("Missing 'template'".to_string()))?
            .children()
            .ok_or(Common("Empty 'template'".to_string()))?;
        let title_elements = template_elements
            .get("title")
            .ok_or(Common("'title' absent".to_string()))?
            .children()
            .ok_or(Common("Empty 'title'".to_string()))?;

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

        let nodes = title_elements.nodes();
        for node in nodes {
            let name = node.name().to_string();
            if name == "header" {
                let header_level = node
                    .get("level")
                    .ok_or(Common("Missing 'level'".to_string()))?
                    .value()
                    .as_i64()
                    .ok_or(Common("Invalid 'level'".to_string()))?;
                let entries = node.entries();
                for entry in entries {
                    if entry.name().is_none() {
                        let header_text = entry
                            .value()
                            .as_string()
                            .ok_or(Common("Invalid text".to_string()))?;
                        let mut resolved_text = header_text.to_string();
                        for (key, value) in &params {
                            resolved_text = resolved_text.replace(&format!("$P{{{}}}", key), value);
                        }
                        let header_element = Header {
                            text: resolved_text,
                            level: header_level as u8,
                        };
                        elements.push(header_element);
                    }
                }
            }
            if name == "image" {
                let src = node
                    .get("src")
                    .ok_or(Common("Missing 'src'".to_string()))?
                    .value()
                    .as_string()
                    .ok_or(Common("Invalid 'src'".to_string()))?;
                let _width = node
                    .get("width")
                    .ok_or(Common("Missing 'width'".to_string()))?
                    .value()
                    .as_i64()
                    .ok_or(Common("Invalid 'width'".to_string()))?;
                let _height = node
                    .get("height")
                    .ok_or(Common("Missing 'height'".to_string()))?
                    .value()
                    .as_i64()
                    .ok_or(Common("Invalid 'height'".to_string()))?;
                let image_bytes = std::fs::read(src)
                    .unwrap_or_else(|_| panic!("Failed to read image file: {}", src));
                let image_bytes = Bytes::from(image_bytes);

                let image_element = Element::Image(ImageData::new(
                    image_bytes,
                    "".to_string(),
                    "".to_string(),
                    ImageType::default().to_string(),
                    ImageAlignment::default().to_string(),
                    ImageDimension::default(),
                ));
                elements.push(image_element);
            }
        }

        let column_header_children = template_elements
            .get("column_header")
            .ok_or(Common("Missing 'column_header'".to_string()))?
            .children()
            .ok_or(Common("Empty 'column_header'".to_string()))?;
        let columns = column_header_children.nodes();
        let mut headers = Vec::new();
        for column in columns {
            let name = column
                .get("name")
                .ok_or(Common("Missing 'name'".to_string()))?
                .value()
                .as_string()
                .ok_or(Common("Invalid 'name'".to_string()))?;
            let width_str = column
                .get("width")
                .ok_or(Common("Missing 'width'".to_string()))?
                .value()
                .to_string();
            let width = width_str.parse::<f32>()?;
            let text_element = Text {
                text: name.to_string(),
                size: 8,
            };
            let header_element = TableHeader {
                element: text_element,
                width,
            };
            headers.push(header_element);
        }

        let row_configs = template_elements
            .get("row")
            .ok_or(Common("Missing 'row'".to_string()))?
            .children()
            .ok_or(Common("Empty 'row'".to_string()))?
            .nodes();

        let mut rows = Vec::new();
        if let Some(data_rows) = data["rows"].as_array() {
            for data_row in data_rows {
                let mut cells = Vec::new();
                for row in row_configs {
                    let value_key = row
                        .entries()
                        .first()
                        .ok_or(Common("Missing 'value'".to_string()))?
                        .value()
                        .as_string()
                        .ok_or(Common("Invalid 'value'".to_string()))?;
                    let field_name = value_key.trim_start_matches("$F(").trim_end_matches(")");
                    if let Some(value) = data_row[field_name].as_str() {
                        let text_element = Text {
                            text: value.to_string(),
                            size: 8,
                        };
                        cells.push(TableCell {
                            element: text_element,
                        });
                    }
                    if let Some(value) = data_row[field_name].as_number() {
                        let value = value.to_string();
                        let text_element = Text {
                            text: value,
                            size: 8,
                        }; // Default font size for cells
                        cells.push(TableCell {
                            element: text_element,
                        });
                    }
                }
                rows.push(TableRow { cells });
            }
        }

        let footer_configs = template_elements
            .get("column_footer")
            .ok_or(Common("Missing 'column_footer'".to_string()))?
            .children()
            .ok_or(Common("Empty 'column_footer'".to_string()))?
            .nodes();
        let mut footer_cells = Vec::new();
        for footer_config in footer_configs {
            let value_key = footer_config
                .entries()
                .first()
                .ok_or(Common("Missing 'value'".to_string()))?
                .value()
                .as_string()
                .ok_or(Common("Invalid 'value'".to_string()))?;
            let mut resolved_text = value_key.to_string();
            for (key, value) in &params {
                resolved_text = resolved_text.replace(&format!("$P{{{}}}", key), value);
            }
            if resolved_text.is_empty() {
                resolved_text = " ".to_string();
            }
            let text_element = Text {
                text: resolved_text,
                size: 8,
            };
            footer_cells.push(TableCell {
                element: text_element,
            });
        }
        if !footer_cells.is_empty() {
            let footer_row = TableRow {
                cells: footer_cells,
            };
            rows.push(footer_row);
        }

        let table_element_with_footer = Table { headers, rows };

        // log::info!("{:?}", table_element_with_footer);
        elements.push(table_element_with_footer);

        // log::info!("{:?}", rows);
        let header = template_elements
            .get("page_header")
            .ok_or(Common("Missing 'page_header'".to_string()))?
            .children()
            .ok_or(Common("Empty 'page_header'".to_string()))?;
        // log::info!("{:?}", header);
        let header_text = header
            .get_args("text")
            .first()
            .ok_or(Common("Missing text".to_string()))?
            .as_string()
            .ok_or(Common("Invalid text".to_string()))?;
        let text_size = header
            .get("text")
            .ok_or(Common("Missing 'header'".to_string()))?
            .get("size")
            .ok_or(Common("Missing 'size'".to_string()))?
            .value()
            .as_i64()
            .ok_or(Common("Invalid 'size'".to_string()))?;
        let text_element = Text {
            text: header_text.to_string(),
            size: text_size as u8,
        };
        page_header.push(text_element);
        // log::info!("{:?}", page_header);

        let footer = template_elements
            .get("page_footer")
            .ok_or(Common("Missing 'page_footer'".to_string()))?
            .children()
            .ok_or(Common("Empty 'page_footer'".to_string()))?;
        // log::info!("{:?}", footer);
        let footer_text = footer
            .get_args("text")
            .first()
            .ok_or(Common("Missing text".to_string()))?
            .as_string()
            .ok_or(Common("Invalid text".to_string()))?;
        let text_size = footer
            .get("text")
            .ok_or(Common("Missing 'footer'".to_string()))?
            .get("size")
            .ok_or(Common("Missing 'size'".to_string()))?
            .value()
            .as_i64()
            .ok_or(Common("Invalid 'size'".to_string()))?;
        let text_element = Text {
            text: footer_text.to_string(),
            size: text_size as u8,
        };
        page_footer.push(text_element);
        debug!("{:?}", page_footer);

        let summary = template_elements
            .get("summary")
            .ok_or(Common("Missing 'summary'".to_string()))?
            .children()
            .ok_or(Common("Empty 'summary'".to_string()))?;
        // log::info!("{:?}", paragraph_config);
        let paragraph = summary
            .get("paragraph")
            .ok_or(Common("Missing 'paragraph'".to_string()))?;
        let text_element = paragraph
            .children()
            .ok_or(Common("Missing children".to_string()))?
            .get("text")
            .ok_or(Common("Missing 'text'".to_string()))?;
        let size = text_element
            .get("size")
            .ok_or(Common("Missing 'size'".to_string()))?
            .value()
            .as_i64()
            .ok_or(Common("Invalid 'size'".to_string()))?;
        let text = paragraph
            .children()
            .ok_or(Common("Missing children".to_string()))?
            .get_args("text")
            .first()
            .ok_or(Common("Missing text".to_string()))?
            .as_string()
            .ok_or(Common("Invalid text".to_string()))?;

        let mut resolved_text = text.to_string();
        for (key, value) in &params {
            resolved_text = resolved_text.replace(&format!("$P{{{}}}", key), value);
        }
        let text_element = Text {
            text: resolved_text,
            size: size as u8,
        };
        let paragraph_element = Paragraph {
            elements: vec![text_element],
        };
        elements.push(paragraph_element);

        let mut document = Document::new(elements);
        document.page_header = page_header;
        document.page_footer = page_footer;
        Ok(document)
    }
}
