# Metatron

![Metatron](https://github.com/igumnoff/metatron/raw/HEAD/logo.png)

**Metatron library: Implementation in Rust of a report generation based on [Shiva library](https://github.com/igumnoff/shiva)**

## Supported report types

- Plain text
- Markdown
- HTML
- PDF

# Usage

Cargo.toml
```toml
[dependencies]
metatron = "0.2.1"
```

```rust
fn main() {
    let template_vec = std::fs::read("report-template.yaml").unwrap();
    let template = std::str::from_utf8(&template_vec).unwrap();
    let data_vec = std::fs::read("report-data.json").unwrap();
    let data = std::str::from_utf8(&data_vec).unwrap();
    let images = HashMap::new();
    let doc = Report::generate(template, data, &images).unwrap();
    let result = shiva::pdf::Transformer::generate(&doc).unwrap();
    std::fs::write("report.pdf",result.0).unwrap();
}
```


## How it works

### report-template.yaml
```yaml
title:
  - header: $P{company_name} Employee Report
    level: 1
page_header:
  - text: Confidential information
    size: 7
column_header:
  - name: Name
    width: 30
  - name: Age
    width: 10
  - name: Salary
    width: 20
row:
  - value: $F(name)
  - value: $F(age)
  - value: $F(salary)
column_footer:
  - value: "Average:"
  - value: $P{average_age}
  - value: $P{average_salary}
page_footer:
  - text: "Tel: +1 123 456 789"
    size: 7
summary:
  - paragraph:
    - text: "Company address: $P{company_address}"
      size: 10
```

### report-data.json
```json
{
   "rows": [
     {
       "name": "John",
       "age": 25,
       "salary": 50000
     },
     {
       "name": "Jane",
       "age": 30,
       "salary": 60000
     },
     {
       "name": "Jim",
       "age": 35,
       "salary": 70000
     }
   ],
   "params": {
     "company_name": "ABCDFG Ltd",
     "company_address": "1234 Elm St, Springfield, IL 62701",
     "average_age": 30,
     "average_salary": 60000
   }
}

```


### Generated report

![PDF](https://github.com/igumnoff/metatron/raw/HEAD/pdf.png)


# TODO
- Add image support
- CLI
- Rest API server