use std::cell::RefCell;

use chrono::{Datelike, NaiveDate, Weekday};
use indexmap::IndexMap;
use ndarray::{Array1, Array2, Axis};
use numfmt::{Formatter, Precision, Scales};

use crate::DataValue;

#[derive(Debug, Default)]
pub struct TableData {
    name: String,
    rows: Array2<DataValue>,
    pub column_metas: IndexMap<String, u32>,
    formatter: RefCell<Formatter>,
}

impl TableData {
    pub fn new(name: &str, rows: usize, column_metas: IndexMap<String, u32>) -> Self {
        let len = column_metas.len();

        let meta = column_metas
            .iter()
            .map(|(k, width)| {
                let k = k.trim();
                let k = if k.len() as u32 > *width {
                    &k[0..*width as usize]
                } else {
                    k
                };
                (k.to_string(), *width)
            })
            .collect::<IndexMap<String, u32>>();

        assert!(meta.len() == len, "{:#?} != {:#?}", column_metas, meta);

        // you probably want to change in "From"
        let formatter = Formatter::new()
            .separator(',')
            .unwrap()
            .scales(Scales::short())
            //.precision(Precision::Significance(5))
            .precision(Precision::Decimals(2));

        TableData {
            name: name.to_string(),
            rows: Array2::default((rows, meta.len())),
            column_metas: meta,
            formatter: RefCell::new(formatter),
        }
    }

    pub fn add_row(&mut self, row: ndarray::ArrayView1<DataValue>) {
        let mut rows: Vec<Array1<_>> = self
            .rows
            .axis_iter(ndarray::Axis(0))
            .map(|row| row.to_owned())
            .collect();

        rows.push(row.to_owned());

        self.rows = ndarray::stack(
            ndarray::Axis(0),
            &rows.iter().map(|r| r.view()).collect::<Vec<_>>(),
        )
        .unwrap();
    }

    pub fn delete_row(&mut self, index: usize) {
        let rows: Vec<_> = self
            .rows
            .axis_iter(ndarray::Axis(0))
            .enumerate()
            .filter_map(|(i, row)| {
                if i != index {
                    Some(row.to_owned())
                } else {
                    None
                }
            })
            .collect();

        if rows.is_empty() {
            // If deleting last row → make empty 2D array with same number of columns
            let empty = ndarray::Array2::from_shape_vec((0, self.rows.ncols()), vec![]).unwrap();
            self.rows = empty;
        } else {
            self.rows = ndarray::stack(
                ndarray::Axis(0),
                &rows.iter().map(|r| r.view()).collect::<Vec<_>>(),
            )
            .unwrap();
        }
    }

    // Method to iterate over rows one at a time
    pub fn iter_rows(&self) -> impl Iterator<Item = ndarray::ArrayView1<'_, DataValue>> {
        self.rows.axis_iter(Axis(0))
    }

    pub fn len(&self) -> usize {
        self.rows.nrows()
    }
    pub fn width(&self) -> usize {
        self.rows.ncols()
    }

    pub fn column(&self, index: usize) -> ndarray::ArrayView1<'_, DataValue> {
        self.rows.column(index)
    }

    // Method to set a value at a specific row and column
    pub fn set_value<T: Into<DataValue> + std::fmt::Debug>(
        &mut self,
        row: usize,
        col: usize,
        value: T,
    ) {
        assert!(
            col < self.column_metas.len(),
            "column:{col} out of range {}",
            self.column_metas.len()
        );
        assert!(row < self.rows.len());
        self.rows[(row, col)] = value.into();
    }

    // Method to get a value at a specific row and column
    pub fn get_value(&self, row: usize, col: usize) -> Option<&DataValue> {
        assert!(
            col < self.column_metas.len() && row < self.rows.len(),
            "column:{col} out of range {}",
            self.column_metas.len()
        );
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

    pub fn format(&self, key: &str, val: &DataValue) -> String {
        let width = self.column_metas[key] as usize;
        let ret = match val {
            DataValue::I64(val) => {
                format!("{val:>width$}")
            }
            DataValue::OptionI64(val) => {
                if val.is_none() {
                    std::iter::repeat(" ").take(width).collect()
                } else {
                    let val = val.unwrap();
                    format!("{val:>width$}")
                }
            }

            DataValue::U64(val) => {
                format!("{val:>width$}")
            }
            DataValue::OptionU64(val) => {
                if val.is_none() {
                    std::iter::repeat(" ").take(width).collect()
                } else {
                    let val = val.unwrap();
                    format!("{val:>width$}")
                }
            }

            DataValue::F64(val) => {
                let mut fmt = self.formatter.borrow_mut();
                let val = fmt.fmt2(*val);
                format!("{val:>width$}")

                //format!("{val:>width$.2}")
            }
            DataValue::OptionF64(val) => {
                if let Some(val) = val {
                    let mut fmt = self.formatter.borrow_mut();
                    let val = fmt.fmt2(*val);
                    format!("{val:>width$}")
                    //format!("{val:>width$.2}")
                } else {
                    std::iter::repeat(" ").take(width).collect()
                }
            }
            DataValue::F32(val) => {
                format!("{val:>width$.4}")
            }
            DataValue::OptionF32(val) => {
                if let Some(val) = val {
                    format!("{val:>width$.4}")
                } else {
                    std::iter::repeat(" ").take(width).collect()
                }
            }

            DataValue::String(val) => {
                format!("{val:<width$}")
            }
            DataValue::OptionString(val) => {
                if val.is_none() {
                    std::iter::repeat(" ").take(width).collect()
                } else {
                    format!("{:<width$}", val.as_ref().unwrap())
                }
            }
            DataValue::Bool(val) => {
                format!("{val:^width$}")
            }
            DataValue::Date(val) => {
                // Calculate the week number in the month
                let monthly_expiration = if let Some(first_day_of_month) =
                    NaiveDate::from_ymd_opt(val.year(), val.month(), 1)
                {
                    let week_number =
                        ((val.day() + first_day_of_month.weekday().num_days_from_sunday()) / 7) + 1;
                    if week_number == 3 && val.weekday() == Weekday::Fri {
                        "*"
                    } else {
                        " "
                    }
                } else {
                    " "
                };

                format!(
                    "{monthly_expiration}{:<width$}",
                    val.format("%Y-%m-%d"),
                    width = width - 1,
                )
            }
            DataValue::OptionDate(val) => {
                if val.is_none() {
                    std::iter::repeat(" ").take(width).collect()
                } else {
                    let val = val.unwrap();
                    // Calculate the week number in the month
                    let monthly_expiration = if let Some(first_day_of_month) =
                        NaiveDate::from_ymd_opt(val.year(), val.month(), 1)
                    {
                        let week_number =
                            ((val.day() + first_day_of_month.weekday().num_days_from_sunday()) / 7)
                                + 1;
                        if week_number == 3 && val.weekday() == Weekday::Fri {
                            "*"
                        } else {
                            " "
                        }
                    } else {
                        " "
                    };

                    format!(
                        "{monthly_expiration}{:<width$}",
                        val.format("%Y-%m-%d"),
                        width = width - 1,
                    )
                }
            }

            DataValue::Time(val) => {
                format!("{:^width$}", val.format("%H:%M"))
            }
            DataValue::OptionTime(val) => {
                if let Some(val) = val {
                    format!("{:^width$}", val.format("%H:%M"))
                } else {
                    std::iter::repeat(" ").take(width).collect()
                }
            }
            DataValue::DateTime(val) => {
                format!("{:^width$}", val.format("%Y-%m-%d %H:%M"))
            }

            DataValue::OptionDateTime(val) => {
                if let Some(val) = val {
                    format!("{:^width$}", val.format("%Y-%m-%d %H:%M"))
                } else {
                    std::iter::repeat(" ").take(width).collect()
                }
            }

            DataValue::U32(val) => {
                format!("{val:^width$}")
            }
            DataValue::OptionU32(val) => {
                if let Some(val) = val {
                    format!("{val:^width$}")
                } else {
                    std::iter::repeat(" ").take(width).collect()
                }
            }
        };
        if ret.len() > width {
            let mut char_boundary_index = 0;

            // Find the valid character boundary closest to `width`
            for (i, _) in ret.char_indices() {
                if i >= width {
                    break;
                }
                char_boundary_index = i;
            }

            let mut ret = ret[0..char_boundary_index].to_string();

            //let mut ret = ret[0..width].to_string();
            //ret.pop();
            ret += "<";
            return ret;
        }
        ret
    }
}

impl From<(&str, usize, &[(&str, u32)])> for TableData {
    fn from(value: (&str, usize, &[(&str, u32)])) -> Self {
        let v = value
            .2
            .iter()
            .map(|(k, v)| (k.to_string(), *v))
            .collect::<Vec<_>>();

        let meta = v.into_iter().collect::<IndexMap<String, u32>>();
        assert!(value.2.len() == meta.len(), "{:#?} != {:#?}", value.2, meta);

        let formatter = Formatter::new()
            .separator(',')
            .unwrap()
            .scales(Scales::short())
            //.precision(Precision::Significance(4));
            .precision(Precision::Decimals(2));

        //.precision(Precision::Decimals(2));

        TableData {
            name: value.0.to_string(),
            rows: Array2::default((value.1, value.2.len() as usize)),
            column_metas: meta,
            formatter: RefCell::new(formatter),
        }
    }
}

impl std::fmt::Display for TableData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut tbl = String::new();

        tbl += &format!("{}\n", self.name);

        let header = self
            .column_metas
            .iter()
            .fold(String::new(), |mut acc, (key, size)| {
                let width = *size as usize;
                let mut key = format!("{:^width$}", key.trim());

                if key.len() as u32 > *size {
                    key = key[0..width].to_string();
                }

                acc += &key;

                acc += "|";
                acc
            });
        tbl += &header;
        tbl += "\n";

        let header_line = self
            .column_metas
            .iter()
            .fold(String::new(), |mut acc, (_key, size)| {
                acc += &std::iter::repeat("-")
                    .take(*size as usize)
                    .collect::<String>();

                acc += "+";
                acc
            });
        tbl += &header_line;
        tbl += "\n";

        for r in self.iter_rows() {
            let row = self.column_metas.iter().enumerate().fold(
                String::new(),
                |mut acc, (index, (key, _size))| {
                    acc += &self.format(key, &r[index]);
                    acc += "|";
                    acc
                },
            );
            tbl.push_str(&row);
            tbl.push_str("\n");
        }

        //write!(f, "{}", header + "\n" + &header_line + "\n" + &tbl)
        write!(f, "{tbl}")
    }
}
