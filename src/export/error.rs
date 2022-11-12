use std::{error::Error, fmt};

use ::csv::Error as CsvError;
use xlsxwriter::XlsxError;

// Export error type

#[derive(Debug, Clone, PartialEq)]
pub struct ExportError {
    details: String,
}

impl ExportError {
    pub fn new(msg: &str) -> ExportError {
        ExportError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for ExportError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for ExportError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<XlsxError> for ExportError {
    fn from(error: XlsxError) -> Self {
        ExportError {
            details: error.to_string(),
        }
    }
}

impl From<CsvError> for ExportError {
    fn from(error: CsvError) -> Self {
        ExportError {
            details: error.to_string(),
        }
    }
}
