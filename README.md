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
metatron = "0.1.2"
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
    width: 20
  - name: Age
    width: 5
  - name: Salary
    width: 5
row:
  - value: $F(name)
  - value: $F(age)
  - value: $F(salary)
column_footer:
  - value: "AVERAGE:"
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

***

Confidential information

# ABCDFG Ltd Employee Report

| Name     | Age | Salary |
|----------|-----|--------|
| John     | 25  | 50000  |
| Jane     | 30  | 60000  |
| Jim      | 35  | 70000  |
| AVERAGE: | 30  | 60000  |

Company address: 1234 Elm St, Springfield, IL 62701

Tel: +1 123 456 789

