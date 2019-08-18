use std::fmt;

use csv::WriterBuilder as CsvWriterBuilder;
use pyo3::prelude::*;
use pyo3::PyObjectProtocol;

/// Builds a CSV writer with various configuration knobs.
/// 
/// This builder can be used to tweak the field delimiter, record
/// terminator and more. Once a CSV `Writer` is build, its configuration
/// cannot be changed.
#[pyclass]
pub struct WriterBuilder {
    inner: CsvWriterBuilder,
}

impl fmt::Display for WriterBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}

#[pymethods]
impl WriterBuilder {
    #[new]
    fn new(obj: &PyRawObject) {
        obj.init(WriterBuilder {
            inner: CsvWriterBuilder::default(),
        });
    }

    /// Set the capacity (in bytes) of the internal buffer used in the
    /// CSV writer. This defaults to a reasonable setting.
    pub fn buffer_capacity(mut slf: PyRefMut<Self>, py: Python, capacity: usize) -> PyResult<PyObject> {
        slf.inner.buffer_capacity(capacity);
        Ok(slf.into_object(py))
    }

    /// The field delimiter to use when writing CSV.
    /// 
    /// The default is `'b,'`.
    pub fn delimiter(mut slf: PyRefMut<Self>, py: Python, delimiter: u8) -> PyResult<PyObject> {
        slf.inner.delimiter(delimiter);
        Ok(slf.into_object(py))
    }

    /// Enable double quote escapes.
    /// 
    /// This is enabled by default, but it may be disabled. When disabled,
    /// quotes in field data are escaped instead of doubled.
    pub fn double_quote(mut slf: PyRefMut<Self>, py: Python, yes: bool) -> PyResult<PyObject> {
        slf.inner.double_quote(yes);
        Ok(slf.into_object(py))
    }

    /// The escape character to use when writing CSV.
    /// 
    /// In some varianted of CSV, quotes are escaped using a special
    /// character like `\` (instead of escaping quotes by doubling them).
    /// 
    /// By default, writing these idiosyncratic escapes is disabled, and is
    /// only used when `double_quote` is disabled.
    pub fn escape(mut slf: PyRefMut<Self>, py: Python, escape: u8) -> PyResult<PyObject> {
        slf.inner.escape(escape);
        Ok(slf.into_object(py))
    }

    /// Whether the number of fields in records is allowed to change or not.
    /// 
    /// When disabled (which is the default), writing CSV data will return an
    /// error if a record is written with a number of fields different from
    /// the number of fields written in a previous record.
    /// 
    /// When enabled, this error checking is turned off.
    pub fn flexible(mut slf: PyRefMut<Self>, py: Python, yes: bool) -> PyResult<PyObject> {
        slf.inner.flexible(yes);
        Ok(slf.into_object(py))
    }

    /// The quote character to use when writing CSV.
    /// 
    /// The default is `b'"'`
    pub fn quote(mut slf: PyRefMut<Self>, py: Python, quote: u8) -> PyResult<PyObject> {
        slf.inner.quote(quote);
        Ok(slf.into_object(py))
    }
}

#[pyproto]
impl PyObjectProtocol for WriterBuilder {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{}", self))
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{}", self))
    }
}