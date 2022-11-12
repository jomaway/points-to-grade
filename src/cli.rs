pub use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(author,version, about,long_about = None)]
pub struct Args {
    // Max amount of points
    //#[arg(short, long)]
    pub points: u16,

    // use half points
    #[arg(long, short='5', help="use half points in grading")]
    pub half_points: bool,

    // Save output to file (.csv, .xlsx)
    #[arg(long,short, help="Save output to file (Supported: .csv, .xlsx)")]
    pub output: Option<String>,

    // Specify a number scale
    #[arg(short,long, value_enum, default_value_t=Scale::German)]
    pub scale: Scale,

    // Use the linear algorithm
    // Which algorithm should be used. If true a linear algorithm will be used. If false the ihk algorithm
    #[arg(long, short, help="Uses the a linear grading algorithm instead of the IHK distribution")]
    pub linear: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Scale {
    // Use german grading scale from 1 to 6.
    German,
    // User grading scale for the FOS with points from 0 to 15 
    GermanFOS,
    // Use English grading scale from A to E.
    English,
}
