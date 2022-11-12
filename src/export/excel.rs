// Excel Exporter
use crate::export::{ExportError, Exporter, GradingList};
use xlsxwriter::{FormatColor, Workbook};

#[derive(Debug, Default)]
pub struct ExcelExporter {
    file: String,
}

impl ExcelExporter {
    pub fn new(output: &str) -> Self {
        Self { file: output.into() }
    }
}

impl Exporter for ExcelExporter {
    fn export(&self, result: GradingList) -> Result<(), ExportError> {
        const COL_POINTS: u16 = 0;
        const COL_GRADE: u16 = 1;
        const ROW_CONTENT_START: u32 = 2;
        let workbook = Workbook::new(&self.file);
        let heading = workbook
            .add_format()
            .set_bold()
            .set_bg_color(FormatColor::Yellow);

        let mut sheet1 = workbook
            .add_worksheet(None)
            .expect("Could not add a sheet to workbook");
        sheet1.merge_range(
            0,
            COL_POINTS,
            0,
            COL_GRADE,
            "Notenschlüssel",
            Some(&heading),
        )?;
        sheet1.write_string(1, COL_POINTS, "Points", None)?;
        sheet1.write_string(1, COL_GRADE, "Grade", None)?;
        for (idx, (points, grade)) in result.data.into_iter().enumerate() {
            sheet1.write_number(
                ROW_CONTENT_START + idx as u32,
                COL_POINTS,
                points as f64,
                None,
            )?;
            sheet1.write_number(
                ROW_CONTENT_START + idx as u32,
                COL_GRADE,
                grade as f64,
                None,
            )?;
        }
        workbook.close().expect("Could not close the workbook");
        Ok(())
    }
}
