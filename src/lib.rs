use pyo3::prelude::*;

pub mod csv;
pub mod errors;

use crate::csv::Reader;
use crate::csv::ReaderBuilder;

#[pymodule]
fn rust_csv(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Reader>()?;
    m.add_class::<ReaderBuilder>()?;

    Ok(())
}
