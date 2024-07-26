use std::str::FromStr;

use bytes::Bytes;
use shiva::core::{Document, DocumentType};

pub fn to_u8arr(document: &Document, document_type: &str) -> Bytes {
    shiva::core::Document::generate(document, DocumentType::from_str(document_type).expect("Document type invalid")).unwrap()
}

mod tests {
    use super::*;
    use shiva::core::Element::{self, Paragraph};

    #[test]
    fn test_to_u8arr() {
        let elements = vec![Element::Text { text: "Hello, World!".to_string(), size: 8 }];
        let paragraph = Paragraph { elements };
        let document = Document::new(vec![paragraph]);
        let result = to_u8arr(&document, &DocumentType::HTML.to_string());
        let _serialized: &[u8] = result.iter().as_slice();

        assert_eq!(result.len(), 66);
    }
}