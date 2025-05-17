use pyo3::exceptions::PyOSError;
use pyo3::PyErr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("{error}")]
    Csv { error: csv::Error },

    #[error("{error}")]
    SerdeJson { error: serde_json::Error },
}

impl From<ApplicationError> for PyErr {
    fn from(err: ApplicationError) -> PyErr {
        PyErr::new::<PyOSError, _>(err.to_string())
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
