use bytes::Bytes;
use shiva::core::{Document, TransformerTrait, DocumentType};
use shiva::core::DocumentType::{Html, Json, Markdown, Pdf, Text};
pub fn to_u8arr(document: &Document, document_type: DocumentType) -> Bytes {
    match document_type {
        Html => {
            shiva::html::Transformer::generate(document).unwrap().0
        }
        Markdown => {
            shiva::markdown::Transformer::generate(document).unwrap().0
        }
        Text => {
            shiva::text::Transformer::generate(document).unwrap().0
        }
        Pdf => {
            shiva::pdf::Transformer::generate(document).unwrap().0
        }
        Json => {
            shiva::json::Transformer::generate(document).unwrap().0
        }
    }

}
mod tests {
    use super::*;
    use shiva::core::Element;
    use bytes::Bytes;
    use shiva::core::DocumentType::{Html, Json, Markdown, Pdf, Text};
    use shiva::core::Element::Paragraph;

    #[test]
    fn test_to_u8arr() {
        let elements = vec![Element::Text { text: "Hello, World!".to_string(), size: 8 }];
        let paragraph = Paragraph { elements };
        let document = Document::new(vec![paragraph]);
        let document_type = Html;
        let result = to_u8arr(&document, document_type);
        let _serialized: &[u8] = result.iter().as_slice();
    }
}