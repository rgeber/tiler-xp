# Tiler XP

> DANGER: This is very fresh software and it's not well tested. Use at your own discretion.

Tiler XP reads an X-Plane FMS flight plan file to create a corridor of ortho existing tiles along your route. Ortho tiles are expected to be in a separate directory and are linked into the X-Plane scenery directory.

This tool does not create tiles but builds on the output created by the brilliant [Ortho4XP](https://github.com/oscarpilote/Ortho4XP) utility.

## Limitations

Tiler XP only works for tiles created with Ortho4XP following the usual naming convention of `zOrtho4XP_<LAT>-<LON>`. This means:

* it does not work with the vstates or other existing tile collections that use a different method
* it does not work for your other scenery. Airports, etc. will not be handled by Tiler XP.
* it was tested only for my own needs on my own Linux setup. No tests have been performed on Mac OS or Windows.

## Usage

At this point there are no binaries. Usage requires you to compile the tool as this version is not yet considered ready for full release. If you have a bit of a developer background go ahead and [install Rust](https://www.rust-lang.org/tools/install) and build the tool:

```bash
cargo build --release
```

Once considered stable/complete (and most of all being tested a little more) I'll a pipeline creating the binaries on github.

### Running Tiler XP

The command itself is called `tiler-xp`. Run `tiler-xp -h` to see the available/required options.

> TODO: Complete list of options once completed

## Motivation

When I was building up a significant coverage of ortho tiles load times for the simulator increased to rather unbearable levels. So I set out to build a tool that would take a flight plan and create a corridor of tiles along the route.

## Contribute

Spotted a mistake? Let me know or contribute code :)

Also any kind of feedback on how it works for you is welcome. Please test on Mac OS and/or Windows.

## Future improvements

It should be possible to make this tool smart enough to work not just with tiles by directory name but analyze every scenery folder allowing users to enable/disable all scenery based on their chosen route(s). A tauri bases UI might also be in the cards but CLI will do the trick for now.
