use std::fs::File;

use csv::{Reader as CsvReader, StringRecord as CsvStringRecord};
use pyo3::prelude::*;
use pyo3::{PyIterProtocol, PyObjectProtocol};

use super::record::CsvRecord;
use crate::errors::ApplicationError;

#[pyclass]
#[derive(Debug)]
pub struct Reader {
    pub reader: CsvReader<File>,
}

#[pymethods]
impl Reader {
    /// Returns a `CsvRecord` returning the first row read by this parse.
    ///
    /// If no row has been read yet, then this will force parsing of the first row.
    ///
    /// If there was a problem parsing the row, or if it wasn't valid UTF-8,
    /// then this returns an error.
    ///
    /// If the underlying reader emits EOF before any data, then this returns
    /// an empty record.
    ///
    /// Note that this method may be used regardless of whether `has_headers`
    /// was enabled (but it is enabled by default).
    fn headers(&mut self) -> PyResult<CsvRecord> {
        let headers = self
            .reader
            .headers()
            .map_err(|err| ApplicationError::from(err))?
            .into();

        Ok(headers)
    }

    #[staticmethod]
    fn from_path(path: &str) -> PyResult<Reader> {
        let reader = CsvReader::from_path(path).map_err(|err| ApplicationError::from(err))?;

        Ok(Reader { reader })
    }
}

#[pyproto]
impl PyObjectProtocol for Reader {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }
}

#[pyproto]
impl PyIterProtocol for Reader {
    fn __iter__(slf: PyRefMut<Self>) -> PyResult<Py<Reader>> {
        Ok(slf.into())
    }

    fn __next__(mut slf: PyRefMut<Self>) -> PyResult<Option<CsvRecord>> {
        let mut record = CsvStringRecord::new();
        let result = slf
            .reader
            .read_record(&mut record)
            .map_err(|err| ApplicationError::from(err))?;

        if result {
            Ok(Some(record.into()))
        } else {
            Ok(None)
        }
    }
}
