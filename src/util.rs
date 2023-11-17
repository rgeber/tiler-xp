use crate::cli::Args;
use crate::AppError;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub(crate) fn check_dirs_exist(args: &Args) -> Result<(), AppError> {
    if !args.xplane_scenery_dir.is_dir() {
        return Err(AppError::NotADir(args.xplane_scenery_dir.clone()));
    }

    if !args.tile_directory.is_dir() {
        return Err(AppError::NotADir(args.tile_directory.clone()));
    }

    if !args.flight_plan.is_file() {
        return Err(AppError::NotAFile(args.flight_plan.clone()));
    }

    Ok(())
}

pub(crate) fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
