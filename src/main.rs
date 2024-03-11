use anyhow::Result;
use clap::Parser;
use std::process::Command;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(version = "1.0", about = "Launch Neovide with selected directory from a fuzzy find", long_about = None)]
struct Args {
    /// Directories to search in for fuzzy finding
    #[arg(
        short,
        long,
        value_name = "DIRECTORY",
        help = "Directories to search in for fuzzy finding"
    )]
    directories: Vec<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.directories.is_empty() {
        eprintln!("No directories provided.");
        std::process::exit(1);
    }

    let dirs = collect_directories(&args.directories)?;

    if dirs.is_empty() {
        eprintln!("No subdirectories found in the provided paths.");
        std::process::exit(1);
    }

    let selected_path = rofi::Rofi::new(&dirs)
        .lines(10)
        .prompt("Select a project to open in Neovide:")
        .run()?;

    launch_neovide(&selected_path)?;

    Ok(())
}

fn collect_directories(paths: &[String]) -> Result<Vec<String>> {
    let mut dirs = Vec::new();
    for path in paths {
        for entry in WalkDir::new(path)
            .min_depth(1)
            .max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_dir() {
                dirs.push(entry.path().to_string_lossy().into_owned());
            }
        }
    }
    Ok(dirs)
}

fn launch_neovide(path: &str) -> Result<()> {
    Command::new("neovide").arg(path).spawn()?.wait()?;
    Ok(())
}
