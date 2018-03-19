#![crate_name = "problem_2"]
use std::fmt;

/// Represents car object with all pertinent parameters
///
/// # param
/// year - car year
/// make - car make
/// model - car model
///
/// # Examples
/// ```
/// use problem_2::Car;
/// let year = "2006";
/// let make = "Hyundai";
/// let model = "Sonata";
/// println!("{} {} {}", year, make, model);
/// ```
struct Car {
    year: String,
    make: String,
    model: String,
}

impl fmt::Display for Car {
    /// Returns printed car with formatting
    ///
    /// # Arguments
    /// year - String that holds value for year of car
    /// make - String that holds value for make of car
    /// model - String that holds value for model of car
    ///
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.year, self.make, self.model)
    }
}



fn main() {
let my_car = Car {year: "1999".to_string(), make: "Ford".to_string(), model: "Escort".to_string()};
println!("Vroom, Vroom, I'm in me mum's {}!", my_car);
}
