use csv::StringRecord;
use pyo3::prelude::*;

/// A single CSV record stored as valid UTF-8 bytes.
#[pyclass(module = "rust_csv")]
#[derive(Debug, Default)]
pub struct CsvRecord {
    record: Vec<String>,
}

impl From<StringRecord> for CsvRecord {
    fn from(string_record: StringRecord) -> CsvRecord {
        let record = string_record.iter().map(str::to_string).collect::<Vec<_>>();
        CsvRecord { record }
    }
}

impl From<&StringRecord> for CsvRecord {
    fn from(string_record: &StringRecord) -> CsvRecord {
        let record = string_record.iter().map(str::to_string).collect::<Vec<_>>();
        CsvRecord { record }
    }
}

#[pymethods]
impl CsvRecord {
    #[getter]
    pub fn record(&self) -> Vec<String> {
        self.record.clone()
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }
}
