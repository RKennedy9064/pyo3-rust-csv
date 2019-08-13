use csv::StringRecord;
use pyo3::prelude::*;
use pyo3::PyObjectProtocol;

/// A single CSV record stored as valid UTF-8 bytes.
#[pyclass]
#[derive(Debug, Default)]
pub struct Record {
    record: Vec<String>,
}

impl From<StringRecord> for Record {
    fn from(string_record: StringRecord) -> Record {
        let record = string_record.iter().map(str::to_string).collect::<Vec<_>>();
        Record { record }
    }
}

impl From<&StringRecord> for Record {
    fn from(string_record: &StringRecord) -> Record {
        let record = string_record.iter().map(str::to_string).collect::<Vec<_>>();
        Record { record }
    }
}

#[pyproto]
impl PyObjectProtocol for Record {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }
}