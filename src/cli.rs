use clap::Parser;
use std::path::PathBuf;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = Some("Tilor XP takes an x-plane flight plan and links ortho tiles along a given route reducing the simulators load times, etc. for tiles that won't be needed for a specific flight."))]
pub struct Args {
    /// X-Plane scenery directory. Usually `<XP_ROOT>/Custom Scenery`
    #[arg(short('x'), long, required = true)]
    pub xplane_scenery_dir: PathBuf,

    /// Ortho tile directory
    #[arg(short('t'), long, required = true)]
    pub tile_directory: PathBuf,

    /// Path to flight plan file
    #[arg(short('p'), long, required = true)]
    pub flight_plan: PathBuf,

    /// Set the amount of tiles around each waypoint
    #[arg(short('w'), long, default_value_t = 2)]
    pub corridor_width: i32,
    // /// If set all ortho links will be removed before the application runs.
    // #[arg(short('d'), long, action)]
    // pub delete_unused_links: bool,
    //
    // /// If set Tilor XP deletes the x-plane scenery ini file. Do not use if your setup uses a customized ini file.
    // #[arg(short('i'), long, action)]
    // pub delete_xplane_scenery_ini: bool,
}
