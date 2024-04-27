use shiva::core::{Document, TransformerTrait, DocumentType};
use shiva::core::DocumentType::{Html, Json, Markdown, Pdf};


pub trait DocumentSerialization{
    fn document_to_u8arr(document: &Document) -> &[u8];
}

pub trait BinarySerializable{
    fn to_u8arr(&self, document_type: DocumentType) -> &[u8];
}

pub struct Text;
pub struct PDF;
pub struct HTML;
pub struct MarkDown;
pub struct JSON;

impl DocumentSerialization for Text{
    fn document_to_u8arr(document: &Document) -> &[u8]{
        let binary_as_bytes = shiva::text::Transformer::generate(document).unwrap().0;
        let serialized: &[u8] = binary_as_bytes.iter().as_slice();
        return serialized;
    }
}

impl DocumentSerialization for PDF{
    fn document_to_u8arr(document: &Document) -> &[u8]{
        let binary_as_bytes = shiva::pdf::Transformer::generate(document).unwrap().0;
        let serialized: &[u8] = binary_as_bytes.iter().as_slice();
        return serialized;
    }
}

impl DocumentSerialization for HTML{
    fn document_to_u8arr(document: &Document) -> &[u8]{
        let binary_as_bytes = shiva::html::Transformer::generate(document).unwrap().0;
        let serialized: &[u8] = binary_as_bytes.iter().as_slice();
        return serialized;
    }
}

impl DocumentSerialization for MarkDown{
    fn document_to_u8arr(document: &Document) -> &[u8]{
        let binary_as_bytes = shiva::markdown::Transformer::generate(document).unwrap().0;
        let serialized: &[u8] = binary_as_bytes.iter().as_slice();
        return serialized;
    }
}

impl DocumentSerialization for JSON{
    fn document_to_u8arr(document: &Document) -> &[u8]{
        let binary_as_bytes = shiva::json::Transformer::generate(document).unwrap().0;
        let serialized: &[u8] = binary_as_bytes.iter().as_slice();
        return serialized;
    }
}

enum DocumentSerializationType{
    Text(Text),
    PDF(PDF),
    HTML(HTML),
    MarkDown(MarkDown),
    JSON(JSON),
}

pub fn shiva_enum_doc_type_to_metatron_doc_type(document_type: DocumentType) -> DocumentSerializationType {
    let document_serializator: dyn DocumentSerialization = match document_type {
        Html => {HTML},
        Markdown => {MarkDown},
        Text => {Text},
        Pdf => {PDF},
        Json => {JSON},
    };
    return document_serializator;
}

impl BinarySerializable for Document{
    fn to_u8arr(&self, document_type: DocumentType) -> &[u8]{
        let document_serializator= shiva_enum_doc_type_to_metatron_doc_type(document_type);
        let serialized_document = document_serializator.to_u8arr(self);
        return serialized_document;
    }
}
