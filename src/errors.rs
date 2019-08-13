use failure::Fail;
use pyo3::exceptions::OSError;
use pyo3::PyErr;

#[derive(Debug, Fail)]
pub enum ApplicationError {
    #[fail(display = "{}", error)]
    Csv { error: csv::Error },

    #[fail(display = "{}", error)]
    SerdeJson { error: serde_json::Error },
}

impl From<ApplicationError> for PyErr {
    fn from(err: ApplicationError) -> PyErr {
        OSError::py_err(err.to_string())
    }
}

impl From<csv::Error> for ApplicationError {
    fn from(err: csv::Error) -> ApplicationError {
        ApplicationError::Csv { error: err }
    }
}

impl From<serde_json::Error> for ApplicationError {
    fn from(err: serde_json::Error) -> ApplicationError {
        ApplicationError::SerdeJson { error: err }
    }
}
