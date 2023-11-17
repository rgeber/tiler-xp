pub mod cli;
pub(crate) mod flight_plan;
pub(crate) mod tiles;
pub(crate) mod util;

use crate::cli::Args;
use crate::flight_plan::parse_flight_plan;
use crate::tiles::{flight_plan_to_tile_names, link_tiles};
use crate::util::check_dirs_exist;
use std::num::{ParseFloatError, ParseIntError};
use std::path::PathBuf;
use thiserror::Error;

pub fn run(args: Args) -> Result<(), AppError> {
    // dbg!(&args);

    if std::env::consts::OS == "ios" || std::env::consts::OS == "android" {
        return Err(AppError::UnsupportedOS);
    }

    check_dirs_exist(&args)?;
    let flight_plan = parse_flight_plan(&args.flight_plan)?;

    let tiles = flight_plan_to_tile_names(flight_plan, &args.corridor_width)?;

    link_tiles(&args.tile_directory, &args.xplane_scenery_dir, &tiles);

    Ok(())
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Directory does not exist: {0}")]
    DirDoesNotExist(PathBuf),

    #[error("File `{0}` does not exist or is not a file.")]
    NotAFile(PathBuf),

    #[error("File `{0}` does not exist or is not a directory.")]
    NotADir(PathBuf),

    #[error("Error parsing flightplan")]
    FlightPlanError,

    #[error("Error converting String to Float.")]
    ConvertToFloatError {
        #[from]
        source: ParseFloatError,
    },

    #[error("Error converting String to Integer.")]
    ConvertToIntError {
        #[from]
        source: ParseIntError,
    },

    #[error("Io Error.")]
    IoError {
        #[from]
        source: std::io::Error,
    },

    #[error("Your OS is not supported.")]
    UnsupportedOS,

    #[error("An unknown error occured.")]
    UnknownError,
}
