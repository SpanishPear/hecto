use crate::Row;

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>
}

impl Document {
    pub fn open() -> Self {
        let mut rows: Vec<Row> = Vec::new();
        rows.push(Row::from("Hello world!"));
        Self {
            rows
        }
    }
}
