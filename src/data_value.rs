#[derive(Debug, Clone)]
pub enum DataValue {
    i64(i64),
    f64(f64),
    u64(u64),
    Bool(bool),
    String(String),
}

impl Default for DataValue {
    fn default() -> Self {
        DataValue::i64(0)
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

// Implement From<i64> for DataValue
impl From<i64> for DataValue {
    fn from(value: i64) -> Self {
        DataValue::i64(value)
    }
}

// Implement From<f64> for DataValue
impl From<f64> for DataValue {
    fn from(value: f64) -> Self {
        DataValue::f64(value)
    }
}

// Implement From<u64> for DataValue
impl From<u64> for DataValue {
    fn from(value: u64) -> Self {
        DataValue::u64(value)
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

// Implement From<&str> for DataValue for convenience
impl From<&str> for DataValue {
    fn from(value: &str) -> Self {
        DataValue::String(value.to_string())
    }
}
