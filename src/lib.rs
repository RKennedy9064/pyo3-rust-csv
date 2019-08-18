use pyo3::prelude::*;

pub mod csv;
pub mod errors;

use crate::csv::Reader;
use crate::csv::ReaderBuilder;
use crate::csv::WriterBuilder;

#[pymodule]
fn rust_csv(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Reader>()?;
    m.add_class::<ReaderBuilder>()?;
    m.add_class::<WriterBuilder>()?;

    Ok(())
}
