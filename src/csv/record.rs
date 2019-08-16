use csv::StringRecord;
use pyo3::prelude::*;
use pyo3::{
    exceptions::{IndexError, ValueError},
    types::PyAny,
    PyObjectProtocol, PySequenceProtocol,
};

/// A single CSV record stored as valid UTF-8 bytes.
#[pyclass]
#[derive(Debug, Default)]
pub struct Record {
    elements: Vec<String>,
}

impl From<StringRecord> for Record {
    fn from(string_record: StringRecord) -> Record {
        let elements = string_record.iter().map(str::to_string).collect::<Vec<_>>();
        Record { elements }
    }
}

#[pyproto]
impl PyObjectProtocol for Record {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }
}

#[pyproto]
impl PySequenceProtocol for Record {
    fn __len__(&'p self) -> PyResult<usize> {
        Ok(self.elements.len())
    }

    fn __getitem__(&self, idx: isize) -> PyResult<String> {
        self.elements
            .get(idx as usize)
            .map(String::to_string)
            .ok_or(IndexError::py_err("list index out of range"))
    }

    fn __setitem__(&mut self, idx: isize, value: &str) -> PyResult<()> {
        self.elements[idx as usize] = value.to_string();
        Ok(())
    }

    fn __delitem__(&'p mut self, idx: isize) -> PyResult<()> {
        if (idx < self.elements.len() as isize) && (idx >= 0) {
            self.elements.remove(idx as usize);
            Ok(())
        } else {
            Err(IndexError::py_err("list index out of range"))
        }
    }

    fn __contains__(&self, other: &PyAny) -> PyResult<bool> {
        match String::extract(other) {
            Ok(ref x) => Ok(self.elements.contains(x)),
            Err(_) => Ok(false),
        }
    }

    fn __concat__(&self, other: &Self) -> PyResult<Self> {
        let mut elements = self.elements.clone();
        elements.extend_from_slice(&other.elements);
        Ok(Self { elements })
    }

    fn __repeat__(&self, count: isize) -> PyResult<Self> {
        if count >= 0 {
            let mut elements = Vec::with_capacity(self.elements.len() * count as usize);
            for _ in 0..count {
                elements.extend(self.elements.clone());
            }
            Ok(Self { elements })
        } else {
            Err(ValueError::py_err("invalid repeat count"))
        }
    }
}
