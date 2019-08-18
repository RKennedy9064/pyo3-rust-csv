use std::fmt;
use std::fs::File;

use csv::{Reader as CsvReader, StringRecord};
use pyo3::prelude::*;
use pyo3::{PyIterProtocol, PyObjectProtocol};

use super::position::Position;
use super::record::Record;

use crate::errors::ApplicationError;

#[pyclass]
#[derive(Debug)]
pub struct Reader {
    pub inner: CsvReader<File>,
}

impl fmt::Display for Reader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}

#[pymethods]
impl Reader {
    /// Create a new CSV parser with a default configuration for the given
    /// file path.
    ///
    /// To customize parsing, use `ReaderBuilder`.
    #[staticmethod]
    pub fn from_path(path: &str) -> PyResult<Reader> {
        let inner = CsvReader::from_path(path).map_err(|err| ApplicationError::from(err))?;

        Ok(Reader { inner })
    }

    /// Returns true if and only if this reader has been configured
    /// to interpret the first record as a header record.
    pub fn has_headers(&self) -> PyResult<bool> {
        Ok(self.inner.has_headers())
    }

    /// Returns a `Record` returning the first row read by this parse.
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
    pub fn headers(&mut self) -> PyResult<Record> {
        let headers = self
            .inner
            .headers()
            .map_err(|err| ApplicationError::from(err))?
            .clone();

        Ok(headers.into())
    }

    /// Returns true if and only if this reader has been exhausted.
    ///
    /// When this returns true, no more records can be read from this
    /// reader (unless is has been seeked to another position).
    pub fn is_done(&self) -> PyResult<bool> {
        Ok(self.inner.is_done())
    }

    /// Return the current position of this CSV reader.
    ///
    /// The byte offset in the position returned can be used to `seek`
    /// this reader. In particular, seeking to a position returned here
    /// on the same data will result in parsing the same subsequent record.
    pub fn position(&self) -> PyResult<Position> {
        Ok(self.inner.position().clone().into())
    }

    /// Seeks the underlying reader to the position given.
    ///
    /// This comes with a few caveats:
    ///
    /// * Any internal buffer associated with this reader is cleared.
    /// * If the given position does not correspond to a position
    ///   immediately before the start of a record, then the behavior
    ///   of this reader is undefined.
    /// * Any special logic that skips the first record in the CSV reader
    ///   when reading or iterating over records is disabled.
    ///
    /// If the given position has a byte offset equivalent to the current
    /// position, then no seeking is performed.
    ///
    /// If the header row has not already been read, then this will attempt
    /// to read the header row before seeking. Therefore, it is possible
    /// that this returns an error associated with reading CSV data.
    ///
    /// Note that seeking is performed based only on the byte offset in the
    /// given position. Namely, the record or line numbers in the position
    /// may be incorrect, but this will cause any future position generated
    /// by this CSV reader to be similarly incorrect.
    pub fn seek(&mut self, pos: &Position) -> PyResult<()> {
        Ok(self
            .inner
            .seek(pos.inner.clone())
            .map_err(|err| ApplicationError::from(err))?)
    }
}

#[pyproto]
impl PyObjectProtocol for Reader {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{}", self))
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{}", self))
    }
}

#[pyproto]
impl PyIterProtocol for Reader {
    fn __iter__(slf: PyRefMut<Self>) -> PyResult<PyObject> {
        let py = unsafe { Python::assume_gil_acquired() };
        Ok(slf.to_object(py))
    }

    fn __next__(mut slf: PyRefMut<Self>) -> PyResult<Option<Record>> {
        let mut record = StringRecord::new();
        let result = slf
            .inner
            .read_record(&mut record)
            .map_err(|err| ApplicationError::from(err))?;

        if result {
            Ok(Some(record.into()))
        } else {
            Ok(None)
        }
    }
}
