use pyo3::prelude::*;

mod csv;
mod errors;

use crate::csv::CsvRecord;
use crate::csv::Reader;
use crate::csv::ReaderBuilder;

#[pymodule]
fn rust_csv(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Reader>()?;
    m.add_class::<ReaderBuilder>()?;
    m.add_class::<CsvRecord>()?;
    Ok(())
}
