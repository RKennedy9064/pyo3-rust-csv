use std::fs::File;

use csv::{Reader as CsvReader, StringRecord as CsvStringRecord};
use pyo3::prelude::*;

use super::record::CsvRecord;
use crate::errors::ApplicationError;

#[pyclass(module = "rust_csv")]
#[derive(Debug)]
pub struct Reader {
    pub(crate) reader: CsvReader<File>,
}

#[pymethods]
impl Reader {
    #[staticmethod]
    pub fn from_path(path: &str) -> PyResult<Self> {
        let reader = CsvReader::from_path(path).map_err(ApplicationError::from)?;
        Ok(Reader { reader })
    }

    pub fn headers(&mut self) -> PyResult<CsvRecord> {
        let headers = self.reader.headers().map_err(ApplicationError::from)?;
        Ok(headers.into())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }

    fn __iter__(slf: PyRefMut<Self>) -> PyResult<Py<Reader>> {
        Ok(slf.into())
    }

    fn __next__(mut slf: PyRefMut<Self>) -> PyResult<Option<CsvRecord>> {
        let mut record = CsvStringRecord::new();
        let result = slf
            .reader
            .read_record(&mut record)
            .map_err(ApplicationError::from)?;

        if result {
            Ok(Some(record.into()))
        } else {
            Ok(None)
        }
    }
}
