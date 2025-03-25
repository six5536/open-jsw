use std::{fs, process::Command};

use crate::{
    Error, Result,
    cli::{
        BuildArgs, DistArgs, RunArgs,
        validation::{print_build_targets, print_run_targets},
    },
    project::{cargo_path, dist_dir, get_packages, project_root, release_dir},
};

pub fn info() -> Result<()> {
    let packages = get_packages();

    // println!("{} {}\n", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    println!();
    println!("[paths]");
    println!("root = {}", project_root().to_string_lossy());
    println!("dist = {}", dist_dir().to_string_lossy());
    println!();
    print_build_targets(&packages);

    Ok(())
}

pub fn build(args: &BuildArgs) -> Result<()> {
    let packages = get_packages();

    if let Some(target) = &args.target {
        // let status = Command::new(cargo_path())
        //     .current_dir(project_root())
        //     .args(["run", "--bin", target])
        //     .status()?;
    } else {
        println!();
        print_build_targets(&packages);
    }

    // log::info!("Converting: {:?}", args.input);

    // dist()?;
    // let input_path = &args.input;
    // // let file = File::open(path)
    // //     .with_context(|| format!("Failed to load conversion input '{:?}'", path))?;
    // // let res = convert(file).with_context(|| format!("Failed to convert '{:?}'", path))?;

    // let raw_game = JswRawGame::from_file(input_path)?;
    // for room in &raw_game.rooms {
    //     println!("{} - {:?}", room.room_no, room.name);
    // }
    // // println!("{:?}", raw_game.rooms);

    // let converter = RawToTiledConverter;

    // let game = converter.convert(&raw_game)?;
    // // println!("{:?}", game);

    // let json = open_jsw_tiled::serialize_map(&game.map)?;

    // // Write the converted game to a file
    // if let Some(output_path) = &args.output {
    //     fs::write(output_path.as_path(), &json)?;
    // }

    // fs::write(output_path.as_path(), &json)?;

    Ok(())
}

pub fn run(args: &RunArgs) -> Result<()> {
    let packages = get_packages();

    if let Some(target) = &args.target {
        // let status = Command::new(cargo_path())
        //     .current_dir(project_root())
        //     .args(["run", "--bin", target])
        //     .status()?;
    } else {
        println!();
        print_run_targets(&packages);
    }
    // log::info!("Reading Tiled map: {:?}", args.input);

    // let path = &args.input;
    // // let file = File::open(path)
    // //     .with_context(|| format!("Failed to load conversion input '{:?}'", path))?;
    // // let res = convert(file).with_context(|| format!("Failed to convert '{:?}'", path))?;

    // // let file = File::open(path)?;
    // let data = fs::read_to_string(path)?;

    // let res = open_jsw_tiled::deserialize_map(&data)?;
    // // for room in res.rooms {
    // //     println!("{} - {:?}", room.room_no, room.name);
    // // }
    // println!("{:?}", res);

    Ok(())
}

pub fn dist(args: &DistArgs) -> Result<()> {
    let packages = get_packages();

    if let Some(target) = &args.target {
        // Build the target dist directory
        // let dist_dir = dist_dir();
        let target_dist_dir = dist_dir();
        let target_dist_bin = target_dist_dir.join(target);
        let target_release_bin = release_dir().join(target);

        println!("target_dist_dir: {:?}", target_dist_dir);
        println!("target_release_bin: {:?}", target_release_bin);

        // Remove and recreate the dist directory for the target
        fs::remove_dir_all(&target_dist_dir).ok();
        fs::create_dir_all(&target_dist_dir)?;

        // Build the release binary
        Command::new(cargo_path())
            .current_dir(project_root())
            .args(["build", "--release", "--bin", target])
            .status()?;

        // Copy the binary to the dist directory
        fs::copy(&target_release_bin, &target_dist_bin).map_err(|err| {
            Error::Text(format!(
                "failed to copy {} => {}: {:?}",
                target_release_bin.to_string_lossy(),
                target_dist_bin.to_string_lossy(),
                err
            ))
        })?;

        // if check_strip_exists() {
        println!("stripping the binary");
        Command::new("strip")
            .arg(&target_dist_bin)
            .status()
            .map_err(|err| Error::Text(format!("failed to strip binary: {:?}", err)))?;
        // } else {
        //     Err(Error::Text("no `strip` utility found".into()))?;
        // }
    } else {
        println!();
        print_build_targets(&packages);
    }

    Ok(())
}

pub fn convert_mm() -> Result<()> {
    let status = Command::new(cargo_path())
        .current_dir(project_root())
        .args([
            "run",
            "--bin",
            "jsw_tool",
            "convert",
            "./resources/mm/bin/mm.tzx",
            "./resources/mm/game",
        ])
        .status()?;

    if !status.success() {
        Err(Error::Text("cargo run failed".into()))?;
    }

    Ok(())
}

pub fn convert_jsw() -> Result<()> {
    let status = Command::new(cargo_path())
        .current_dir(project_root())
        .args([
            "run",
            "--bin",
            "jsw_tool",
            "convert",
            "./resources/jsw/bin/jsw.tzx",
            "./resources/jsw/game",
        ])
        .status()?;

    if !status.success() {
        Err(Error::Text("cargo run failed".into()))?;
    }

    Ok(())
}

pub fn convert_jsw2() -> Result<()> {
    let status = Command::new(cargo_path())
        .current_dir(project_root())
        .args([
            "run",
            "--bin",
            "jsw_tool",
            "convert",
            "./resources/jsw2/bin/jsw2.tzx",
            "./resources/jsw2/game",
        ])
        .status()?;

    if !status.success() {
        Err(Error::Text("cargo run failed".into()))?;
    }

    Ok(())
}

// TODO: Move to a separate module
// pub fn check_strip_exists() -> bool {
//     Command::new("strip")
//         .arg("--version")
//         .stdout(std::process::Stdio::null())
//         .status()
//         .is_ok()
// }
