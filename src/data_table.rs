use indexmap::IndexMap;
use ndarray::{Array2, Axis};

use crate::DataValue;

#[derive(Debug)]
pub struct TableData {
    rows: Array2<DataValue>,
    pub column_metas: IndexMap<String, usize>,
}

impl TableData {
    pub fn new(rows: usize, column_metas: IndexMap<String, usize>) -> Self {
        let column_metas = column_metas
            .into_iter()
            .map(|(k, width)| {
                let k = k.trim();
                let k = if k.len() > width { &k[0..width] } else { k };
                (k.to_string(), width)
            })
            .collect::<IndexMap<String, usize>>();

        TableData {
            rows: Array2::default((rows, column_metas.len())),
            column_metas,
        }
    }

    // Method to iterate over rows one at a time
    pub fn iter_rows(&self) -> impl Iterator<Item = ndarray::ArrayView1<DataValue>> {
        self.rows.axis_iter(Axis(0))
    }

    // Method to set a value at a specific row and column
    pub fn set_value<T: Into<DataValue>>(&mut self, row: usize, col: usize, value: T) {
        assert!(col < self.column_metas.len() && row < self.rows.len());
        self.rows[(row, col)] = value.into();
    }

    // Method to get a value at a specific row and column
    pub fn get_value(&self, row: usize, col: usize) -> Option<&DataValue> {
        assert!(col < self.column_metas.len() && row < self.rows.len());
        self.rows.get((row, col))
    }

    pub fn format_row(&self, row: ndarray::ArrayView1<DataValue>) -> String {
        self.column_metas
            .iter()
            .enumerate()
            .fold(String::new(), |mut acc, (index, (key, __))| {
                acc += &self.format(key, &row[index]);
                acc += "│";
                acc
            })
    }

    // pub fn format_header(&self) -> String {
    //     self.column_metas
    //         .iter()
    //         .fold(String::new(), |mut acc, (key, _width)| {
    //             acc += &self.format(key, &DataValue::String(key.to_owned()));
    //             acc += " | ";
    //             acc
    //         })
    // }

    pub fn format(&self, key: &str, val: &DataValue) -> String {
        match val {
            DataValue::i64(val) => {
                format!("{val:width$}", width = self.column_metas[key])
            }
            DataValue::u64(val) => {
                format!("{val:width$}", width = self.column_metas[key])
            }
            DataValue::f64(val) => {
                format!("{val:width$.2}", width = self.column_metas[key])
            }
            DataValue::String(val) => {
                format!("{val:width$}", width = self.column_metas[key])
            }
            DataValue::Bool(val) => {
                format!("{val:^width$}", width = self.column_metas[key])
            }
        }
    }
}

impl From<(usize, &[(&str, usize)])> for TableData {
    fn from(value: (usize, &[(&str, usize)])) -> Self {
        let v = value
            .1
            .iter()
            .map(|(k, v)| (k.to_string(), *v))
            .collect::<Vec<_>>();

        TableData {
            rows: Array2::default((value.0, value.1.len())),
            column_metas: v.into_iter().collect::<IndexMap<String, usize>>(),
        }
    }
}

impl std::fmt::Display for TableData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut tbl = String::new();

        let header = self
            .column_metas
            .iter()
            .fold(String::new(), |mut acc, (key, size)| {
                acc += &(key[0..*size]);
                acc += " | ";
                acc
            });
        let header_line = self
            .column_metas
            .iter()
            .fold(String::new(), |mut acc, (_key, size)| {
                acc += &"--------------------------------------------"[0..*size];
                acc += "-+-";
                acc
            });

        for r in self.iter_rows() {
            let row = self.column_metas.iter().enumerate().fold(
                String::new(),
                |mut acc, (index, (key, __))| {
                    acc += &self.format(key, &r[index]);
                    acc += " | ";
                    acc
                },
            );
            tbl.push_str(&row);
            tbl.push_str("\n");
        }

        write!(f, "{}", header + "\n" + &header_line + "\n" + &tbl)
    }
}
