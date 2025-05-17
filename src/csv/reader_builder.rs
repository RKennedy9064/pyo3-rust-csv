use csv::ReaderBuilder as CsvReaderBuilder;
use pyo3::prelude::*;

use super::reader::Reader;
use crate::errors::ApplicationError;

#[pyclass(module = "rust_csv")]
#[derive(Debug, Default)]
pub struct ReaderBuilder {
    builder: CsvReaderBuilder,
}

#[pymethods]
impl ReaderBuilder {
    #[new]
    fn new() -> Self {
        ReaderBuilder::default()
    }

    fn ascii(mut slf: PyRefMut<Self>) -> PyResult<Py<ReaderBuilder>> {
        slf.builder.ascii();
        Ok(slf.into())
    }

    fn buffer_capacity(mut slf: PyRefMut<Self>, capacity: usize) -> PyResult<Py<ReaderBuilder>> {
        slf.builder.buffer_capacity(capacity);
        Ok(slf.into())
    }

    fn comment(mut slf: PyRefMut<Self>, comment: Option<u8>) -> PyResult<Py<ReaderBuilder>> {
        slf.builder.comment(comment);
        Ok(slf.into())
    }

    fn delimiter(mut slf: PyRefMut<Self>, delimiter: u8) -> PyResult<Py<ReaderBuilder>> {
        slf.builder.delimiter(delimiter);
        Ok(slf.into())
    }

    fn double_quote(mut slf: PyRefMut<Self>, yes: bool) -> PyResult<Py<ReaderBuilder>> {
        slf.builder.double_quote(yes);
        Ok(slf.into())
    }

    fn escape(mut slf: PyRefMut<Self>, escape: Option<u8>) -> PyResult<Py<ReaderBuilder>> {
        slf.builder.escape(escape);
        Ok(slf.into())
    }

    fn flexible(mut slf: PyRefMut<Self>, yes: bool) -> PyResult<Py<ReaderBuilder>> {
        slf.builder.flexible(yes);
        Ok(slf.into())
    }

    fn has_headers(mut slf: PyRefMut<Self>, yes: bool) -> PyResult<Py<ReaderBuilder>> {
        slf.builder.has_headers(yes);
        Ok(slf.into())
    }

    fn quote(mut slf: PyRefMut<Self>, quote: u8) -> PyResult<Py<ReaderBuilder>> {
        slf.builder.quote(quote);
        Ok(slf.into())
    }

    fn quoting(mut slf: PyRefMut<Self>, yes: bool) -> PyResult<Py<ReaderBuilder>> {
        slf.builder.quoting(yes);
        Ok(slf.into())
    }

    fn from_path(&self, path: &str) -> PyResult<Reader> {
        let reader = self
            .builder
            .from_path(path)
            .map_err(ApplicationError::from)?;
        Ok(Reader { reader })
    }
}
