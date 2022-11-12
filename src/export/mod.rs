pub mod csv;
pub mod excel;
pub mod error;

pub use self::excel::ExcelExporter;
use error::ExportError;

use crate::calc::GradingList;
// Export trait

pub trait Exporter {
    fn export(&self, result: GradingList) -> Result<(), ExportError>;
}