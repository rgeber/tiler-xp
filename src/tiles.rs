use crate::flight_plan::FlightPlanEntry;
use crate::AppError;
use itertools::Itertools;
use std::path::PathBuf;
use symlink::symlink_dir;

pub(crate) fn flight_plan_to_tile_names(
    flight_plan: Vec<FlightPlanEntry>,
    corridor_width: &i32,
) -> Result<Vec<String>, AppError> {
    let mut tiles: Vec<String> = vec![];

    for e in flight_plan.iter() {
        let lat = e.lat as i32;
        let lon = e.lon as i32;

        let lat_range = (lat - corridor_width)..(lat + corridor_width + 1);

        for lat in lat_range {
            let lon_range = (lon.clone() - corridor_width)..(lon.clone() + corridor_width + 1);

            for lon in lon_range {
                let lat = match lat < 0 {
                    true => format!("-{:02}", lat * -1),
                    _ => format!("+{:02}", lat),
                };

                let lon = match lon < 0 {
                    true => format!("-{:03}", lon * -1),
                    _ => format!("+{:03}", lon),
                };

                tiles.push(format!("zOrtho4XP_{}{}", lat, lon).into());
            }
        }
    }

    let tiles: Vec<String> = tiles.into_iter().unique().into_iter().map(|e| e).collect();

    Ok(tiles)
}

pub(crate) fn link_tiles(
    tile_path: &PathBuf,
    scenery_path: &PathBuf,
    tiles: &Vec<String>,
) -> Result<(), AppError> {
    for tile in tiles.iter() {
        let tile_path = tile_path.join(tile);
        let scenery_path = scenery_path.join(tile);
        if tile_path.exists() && !scenery_path.exists() {
            println!("Linking: {:?} -> {:?}", tile_path, scenery_path);
            symlink_dir(tile_path, scenery_path)?;
        }
    }

    Ok(())
}
