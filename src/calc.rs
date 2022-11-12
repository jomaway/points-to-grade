use std::{error::Error, fmt};

// Errortype

#[derive(Debug, PartialEq)]
pub struct CalculationError {
    details: String,
}

impl CalculationError {
    pub fn new(msg: &str) -> CalculationError {
        CalculationError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for CalculationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for CalculationError {
    fn description(&self) -> &str {
        &self.details
    }
}

// GradingScale
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum GradingScale {
    German,    // German grading scale with grades from 1 to 6
    GermanFOS, // German FOS/BOS grading scale from 0 to 15 points
    English,   // English grading scale with letters
}

impl Default for GradingScale {
    fn default() -> Self {
        GradingScale::German
    }
}

// GradingAlgorithm type
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum GradingAlgorithm {
    IHK, // 50% is the last 4
    Linear,
}

impl Default for GradingAlgorithm {
    fn default() -> Self {
        GradingAlgorithm::IHK
    }
}

// Grading Calculator

#[derive(Debug, Default)]
pub struct GradeCalculator {
    scale: GradingScale,
    max_points: u16,
    algorithm: GradingAlgorithm,
    use_half_points: bool,
}

impl GradeCalculator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn scale(&mut self, scale: GradingScale) -> &mut Self {
        self.scale = scale;
        self
    }

    pub fn max_points(&mut self, points: u16) -> &mut Self {
        println!("Set max points to {}", points);
        self.max_points = points;
        self
    }

    pub fn algorithm(&mut self, algorithm: GradingAlgorithm) -> &mut Self {
        println!("Use {:?} algorithm", algorithm);
        self.algorithm = algorithm;
        self
    }

    pub fn use_half_points(&mut self, arg: bool) -> &mut Self {
        println!("Show half points");
        self.use_half_points = arg;
        self
    }

    pub fn calc(&self) -> Result<GradingList, CalculationError> {
        let mut grading = Vec::new();
        for point in 0..=self.max_points {
            let grade = self.calc_grade_from_points(point as f32);
            // Add point to the map
            grading.push((point as f32, grade));
            // If use half_points add them too
            if self.use_half_points == true && point != self.max_points {
                let point = point as f32 + 0.5;
                let grade = self.calc_grade_from_points(point);
                grading.push((point, grade));
            }
        }

        Result::Ok(GradingList::from_vec(grading))
    }

    pub fn calc_grade_from_points(&self, points: f32) -> u8 {
        let percentage = self.point_as_percentage(points);
        match self.algorithm {
            GradingAlgorithm::IHK => self.ihk_percentage_to_grade(percentage),
            GradingAlgorithm::Linear => self.linear_percentage_to_grade(percentage),
        }
    }

    fn point_as_percentage(&self, point: f32) -> f32 {
        point / self.max_points as f32
    }

    fn round_float_to_2_decimal_places(&self, float: f32) -> f32 {
        (float * 100.0).round() / 100.0
    }

    fn ihk_percentage_to_grade(&self, percentage: f32) -> u8 {
        //todo!("Add range check for percentage");

        // round percentage to 2 decimal places
        match self.round_float_to_2_decimal_places(percentage) {
            x if x < 0.3 => 6,
            x if x < 0.5 => 5,
            x if x < 0.67 => 4,
            x if x < 0.81 => 3,
            x if x < 0.92 => 2,
            x if x <= 1.0 => 1,
            _ => 1, // Return Grade 1 if percentage is over 100%
        }
    }

    fn linear_percentage_to_grade(&self, percentage: f32) -> u8 {
        (6.0 - 5.0 * percentage).round() as u8
    }
}

#[derive(Debug, Default, Clone)]
pub struct GradingList {
    pub data: Vec<(f32, u8)>,
}

impl GradingList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_vec(vec: Vec<(f32, u8)>) -> Self {
        GradingList { data: vec }
    }

    pub fn add(&mut self, point: f32, grade: u8) {
        // check if point is already in list
        if !self.data.contains(&(point, grade)) {
            self.data.push((point, grade))
        } else {
            println!("What should i do?")
        }
    }

    pub fn to_distribution(&self) {
        unimplemented!("Fail")
    }

    pub fn print(&self) {
        let mut prev_grade: u8 = 6;
        for (p, g) in &self.data {
            if g < &prev_grade {
                prev_grade = *g;
                println!("{}", "-".repeat(10))
            }
            println!("{:>4.1} - {}", p, g);
        }
    }
}

impl Iterator for GradingList {
    type Item = (f32, u8);

    fn next(&mut self) -> Option<Self::Item> {
        let current = (1.0, 1);
        Some(current)
    }
}
