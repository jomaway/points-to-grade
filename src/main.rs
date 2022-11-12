
mod calc;
mod export;
mod cli;


use export::{ExcelExporter, Exporter, csv::CsvExporter};
use log::{debug, error, info};
use calc::{GradingAlgorithm, GradeCalculator};
use cli::{Args, Parser};

use crate::calc::GradingScale;


fn main() {
    // Init logger
    env_logger::init();
    debug!("Init logger.");

    let cli = Args::parse();

    let algorithm = if cli.linear {
        GradingAlgorithm::Linear
    } else {
        GradingAlgorithm::IHK
    };

    let grading = GradeCalculator::new()
        .max_points(cli.points)
        .algorithm(algorithm)
        .use_half_points(cli.half_points)
        .scale(GradingScale::German)
        .calc()
        .expect("Failed grading");

    if let Some(output) = cli.output {
        let suffix = output.split('.').last();
        match suffix {
            Some("csv") => CsvExporter::new(&output).export(grading).expect(format!("Error exporting {}", output).as_str()),
            Some("xlsx") => ExcelExporter::new(&output).export(grading).expect(format!("Error exporting {}", output).as_str()),
            _ => error!("Outout error: No supported suffix found. [Supported: .csv or .xlsx]")
        }
    } else {
        // log println!("No exporter specified!");
        info!("No output file specified");
        println!("Result:");
        grading.print();
    }

}
