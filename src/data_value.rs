use std::fmt::Display;

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

#[derive(Debug, Clone)]
pub enum DataValue {
    I64(i64),
    F64(f64),
    F32(f32),
    U64(u64),
    U32(u32),
    Bool(bool),
    String(String),
    Date(NaiveDate),
    Time(NaiveTime),
    DateTime(NaiveDateTime),
    OptionDate(Option<NaiveDate>),
    OptionTime(Option<NaiveTime>),
    OptionDateTime(Option<NaiveDateTime>),
    OptionF64(Option<f64>),
    OptionF32(Option<f32>),
    OptionI64(Option<i64>),
    OptionU64(Option<u64>),
    OptionU32(Option<u32>),
    OptionString(Option<String>),
}

impl Default for DataValue {
    fn default() -> Self {
        DataValue::I64(0)
    }
}

impl DataValue {
    pub fn value<T>(v: T) -> DataValue
    where
        T: Into<DataValue>,
    {
        v.into()
    }
}

impl Display for DataValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

// Implement From<i64> for DataValue
impl From<i64> for DataValue {
    fn from(value: i64) -> Self {
        DataValue::I64(value)
    }
}

// Implement From<f64> for DataValue
impl From<f64> for DataValue {
    fn from(value: f64) -> Self {
        DataValue::F64(value)
    }
}

impl From<f32> for DataValue {
    fn from(value: f32) -> Self {
        DataValue::F32(value)
    }
}

// Implement From<u64> for DataValue
impl From<u64> for DataValue {
    fn from(value: u64) -> Self {
        DataValue::U64(value)
    }
}

// Implement From<bool> for DataValue
impl From<bool> for DataValue {
    fn from(value: bool) -> Self {
        DataValue::Bool(value)
    }
}

// Implement From<String> for DataValue
impl From<String> for DataValue {
    fn from(value: String) -> Self {
        DataValue::String(value)
    }
}

impl From<Option<String>> for DataValue {
    fn from(value: Option<String>) -> Self {
        DataValue::OptionString(value)
    }
}

// Implement From<&str> for DataValue for convenience
impl From<&str> for DataValue {
    fn from(value: &str) -> Self {
        DataValue::String(value.to_string())
    }
}

impl From<&NaiveDate> for DataValue {
    fn from(value: &NaiveDate) -> Self {
        DataValue::Date(value.clone())
    }
}
impl From<Option<NaiveDate>> for DataValue {
    fn from(value: Option<NaiveDate>) -> Self {
        DataValue::OptionDate(value)
    }
}

impl From<&NaiveTime> for DataValue {
    fn from(value: &NaiveTime) -> Self {
        DataValue::Time(value.clone())
    }
}
impl From<Option<NaiveTime>> for DataValue {
    fn from(value: Option<NaiveTime>) -> Self {
        DataValue::OptionTime(value)
    }
}
impl From<&NaiveDateTime> for DataValue {
    fn from(value: &NaiveDateTime) -> Self {
        DataValue::DateTime(value.clone())
    }
}

impl From<Option<NaiveDateTime>> for DataValue {
    fn from(value: Option<NaiveDateTime>) -> Self {
        DataValue::OptionDateTime(value)
    }
}

impl From<u32> for DataValue {
    fn from(value: u32) -> Self {
        DataValue::U32(value)
    }
}

impl From<Option<f64>> for DataValue {
    fn from(value: Option<f64>) -> Self {
        DataValue::OptionF64(value)
    }
}

impl From<Option<f32>> for DataValue {
    fn from(value: Option<f32>) -> Self {
        DataValue::OptionF32(value)
    }
}

impl From<Option<i64>> for DataValue {
    fn from(value: Option<i64>) -> Self {
        DataValue::OptionI64(value)
    }
}
impl From<Option<u64>> for DataValue {
    fn from(value: Option<u64>) -> Self {
        DataValue::OptionU64(value)
    }
}
impl From<Option<u32>> for DataValue {
    fn from(value: Option<u32>) -> Self {
        DataValue::OptionU32(value)
    }
}
