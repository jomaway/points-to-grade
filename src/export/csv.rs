use super::{ExportError, Exporter, GradingList};
// CSV Exporter

#[derive(Debug, Default)]
pub struct CsvExporter {
    file: String,
}

impl CsvExporter{
    pub fn new(output: &str) -> Self {
        CsvExporter { file: output.into()}
    }
}

impl Exporter for CsvExporter {
    fn export(&self, result: GradingList) -> Result<(), ExportError> {
        let mut wtr = csv::Writer::from_path(&self.file)?;

        for (points, grade) in result.data.into_iter() {
            wtr.serialize((points.to_string(), grade.to_string())).expect("Error writing row to csv file.");
        }
        wtr.flush().expect("Error writing csv file.");

        println!("Writing result to file {}", self.file);
        Ok(())
    }
}
