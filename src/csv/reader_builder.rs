use csv::ReaderBuilder as CsvReaderBuilder;
use pyo3::prelude::*;

use super::reader::Reader;
use crate::errors::ApplicationError;

/// A wrapper around `csv::ReaderBuilder`
///
/// https://docs.rs/csv/1.1.1/csv/struct.ReaderBuilder.html
///
/// Builds a CSV reader with various configuration knobs.
///
/// This builder can be used to tweak the field delimiter, record
/// terminator and more. Once a CSV `Reader` is build, its
/// configuration cannot be changed.
#[pyclass]
#[derive(Debug, Default)]
pub struct ReaderBuilder {
    builder: CsvReaderBuilder,
}

#[pymethods]
impl ReaderBuilder {
    #[new]
    fn new(obj: &PyRawObject) {
        obj.init({ ReaderBuilder::default() });
    }

    /// A convenience method for specifying a configuration to to read
    /// ASCII delimited text.
    ///
    /// This sets the delimiter and record terminator to the ASCII unit
    /// separator `(\x1F)` and record separator `(\x1E)` respectively.
    fn ascii(mut slf: PyRefMut<Self>, py: Python) -> PyResult<PyObject> {
        slf.builder.ascii();
        Ok(slf.into_object(py))
    }

    /// Set the capacity (in bytes) of the buffer used in the CSV reader.
    /// This defaults to a reasonable setting.
    fn buffer_capacity(mut slf: PyRefMut<Self>, py: Python, capacity: usize) -> PyResult<PyObject> {
        slf.builder.buffer_capacity(capacity);
        Ok(slf.into_object(py))
    }

    /// The comment character to use when parsing CSV.
    ///
    /// If the start of a record begins with the byte given here,
    /// then that line is ignored by the CSV parser.
    ///
    /// This is disabled by default.
    fn comment(mut slf: PyRefMut<Self>, py: Python, comment: Option<u8>) -> PyResult<PyObject> {
        slf.builder.comment(comment);
        Ok(slf.into_object(py))
    }

    /// The field to use when parsing CSV.
    ///
    /// The default is `b','`.
    fn delimiter(mut slf: PyRefMut<Self>, py: Python, delimiter: u8) -> PyResult<PyObject> {
        slf.builder.delimiter(delimiter);
        Ok(slf.into_object(py))
    }

    /// Enable double quote escapes.
    ///
    /// This is enabled by default, but it may be disabled. When disabled,
    /// doubled quotes are not interpreted as escapes.
    fn double_quote(mut slf: PyRefMut<Self>, py: Python, yes: bool) -> PyResult<PyObject> {
        slf.builder.double_quote(yes);
        Ok(slf.into_object(py))
    }

    /// The escape character to use when parsing CSV.
    ///
    /// In some variants of CSV, quotes are escaped using a special escape
    /// character like `\` (instead of escaping quotes by doubling them).
    ///
    /// By default, recognizing these idiosyncratic escapes is disabled.
    fn escape(mut slf: PyRefMut<Self>, py: Python, escape: Option<u8>) -> PyResult<PyObject> {
        slf.builder.escape(escape);
        Ok(slf.into_object(py))
    }

    /// Whether the number of fields in records is allowed to change or not.
    ///
    /// When disabled (which is the default), parsing CSV data will return an
    /// error if a record is found with a number of fields different from the
    /// number of fields in a previous record.
    ///
    /// When enabled, this error checking is turned off.
    fn flexible(mut slf: PyRefMut<Self>, py: Python, yes: bool) -> PyResult<PyObject> {
        slf.builder.flexible(yes);
        Ok(slf.into_object(py))
    }

    /// Builds a CSV parser from this configuration that reads data from
    /// the given file path.
    ///
    /// If there was a proglem opening the file at the given path, 
    /// then this returns the corresponding error.
    fn from_path(&self, path: &str) -> PyResult<Reader> {
        let reader = self
            .builder
            .from_path(path)
            .map_err(|err| ApplicationError::from(err))?;
        Ok(Reader { reader })
    }

    /// Whether to treat the first row as a special header row.
    /// 
    /// By default, the first row is treated as a special header row,
    /// which means the header is never returned by any of the record
    /// reading methods or iterators. When this is disabled (`yes` set
    /// to `false`), the first row is not treated specially.
    /// 
    /// Note that the `headers` and `byte_headers` methods are unaffected
    /// by whether this is set. Those methods always return the first record.
    fn has_headers(mut slf: PyRefMut<Self>, py: Python, yes: bool) -> PyResult<PyObject> {
        slf.builder.has_headers(yes);
        Ok(slf.into_object(py))
    }

    /// The quote character to use when parsing CSV.
    /// 
    /// The default is `b'"'`.
    fn quote(mut slf: PyRefMut<Self>, py: Python, quote: u8) -> PyResult<PyObject> {
        slf.builder.quote(quote);
        Ok(slf.into_object(py))
    }

    /// Enable or disable quoting.
    /// 
    /// This is enabled by default, but it may be disabled. When disabled,
    /// quotes are not treated specially.
    fn quoting(mut slf: PyRefMut<Self>, py: Python, yes: bool) -> PyResult<PyObject> {
        slf.builder.quoting(yes);
        Ok(slf.into_object(py))
    }
}
