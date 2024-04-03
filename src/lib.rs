use std::collections::HashMap;
use bytes::Bytes;
use serde_yaml::Value as YValue;
use serde_json::Value as JValue;

use thiserror::Error;

pub struct Report;


impl Report {

    pub fn generate(template: &str, data: &str,  _images: &HashMap<String, Bytes>) -> anyhow::Result<Bytes> {
        let data: YValue = serde_yaml::from_str(template)?;
        let data1 = data.get("title").ok_or(ReportError::Common)?;
        println!("{:?}", data1);
        println!("{:?}", data1.as_sequence().ok_or(ReportError::Common)?.first());
        Ok(Bytes::from(""))
    }

}

#[derive(Error, Debug)]
pub enum ReportError {
    #[error("Report error")]
    Common,
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
