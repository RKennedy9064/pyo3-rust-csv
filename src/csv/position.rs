use std::fmt;

use csv::Position as CsvPosition;
use pyo3::prelude::*;
use pyo3::PyObjectProtocol;

/// A position in CSV data.
/// 
/// A position is used to report errors in CSV data. All positions include
/// the byte offset, line number and record index at which the error occured.
/// 
/// Byte offsets and record indices start at `0`. Line numbers start at `1`
/// 
/// A CSV reader will automatically assign the position of each record.
#[pyclass]
#[derive(Debug)]
pub struct Position {
    pub inner: CsvPosition,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}

impl From<CsvPosition> for Position {
    fn from(csv_position: CsvPosition) -> Position {
        Position {
            inner: csv_position,
        }
    }
}

#[pymethods]
impl Position {
    #[new]
    fn new(obj: &PyRawObject) {
        obj.init(Position {
            inner: CsvPosition::new(),
        });
    }

    /// The byte offset, starting at `0`, of this position.
    pub fn byte(&self) -> PyResult<u64> {
        Ok(self.inner.byte())
    }

    /// The line number, starting at `1`, of this position.
    pub fn line(&self) -> PyResult<u64> {
        Ok(self.inner.line())
    }

    /// The record index, starting with the first record at `0`.
    pub fn record(&self) -> PyResult<u64> {
        Ok(self.inner.record())
    }

    /// Sets the byte offset of this position.
    pub fn set_byte(mut slf: PyRefMut<Self>, py: Python, byte: u64) -> PyResult<PyObject> {
        slf.inner.set_byte(byte);
        Ok(slf.into_object(py))
    }

    /// Sets the line number of this position.
    /// 
    /// If the line number is less then `1`, then this method panics.
    pub fn set_line(mut slf: PyRefMut<Self>, py: Python, line: u64) -> PyResult<PyObject> {
        slf.inner.set_line(line);
        Ok(slf.into_object(py))
    }

    /// Sets the record index of this position.
    pub fn set_record(mut slf: PyRefMut<Self>, py: Python, record: u64) -> PyResult<PyObject> {
        slf.inner.set_record(record);
        Ok(slf.into_object(py))
    }
}

#[pyproto]
impl PyObjectProtocol for Position {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{}", self))
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{}", self))
    }
}
