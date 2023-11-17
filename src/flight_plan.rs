use crate::util::read_lines;
use crate::AppError;
use std::path::PathBuf;

#[derive(Debug)]
pub(crate) struct FlightPlanEntry {
    pub(crate) lat: f32,
    pub(crate) lon: f32,
}

pub(crate) fn parse_flight_plan(
    flight_plan_path: &PathBuf,
) -> Result<Vec<FlightPlanEntry>, AppError> {
    let mut flight_plan: Vec<FlightPlanEntry> = vec![];

    if let Ok(lines) = read_lines(flight_plan_path) {
        for line in lines {
            if let Ok(ip) = line {
                let parts = ip.split(" ");
                let collection = parts.collect::<Vec<&str>>();

                if collection.len() == 6 {
                    flight_plan.push(FlightPlanEntry {
                        lat: String::from(collection[4]).parse()?,
                        lon: String::from(collection[5]).parse()?,
                    })
                }
            }
        }
    }

    Ok(flight_plan)
}
