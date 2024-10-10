use anyhow::Result;
use clap::Parser;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(version = "0.0.1", about = "move all files from sub-folders to root", long_about = None)]
struct Args {
    #[arg(required = true)]
    path: String,
}

impl Args {
    pub fn path_buf(&self) -> PathBuf {
        PathBuf::from(self.path.as_str())
    }
}

fn main() -> Result<()> {
    let args: Args = Args::parse();
    flatten_directory(&args.path_buf());
    remove_directories(&args.path_buf())?;
    Ok(())
}

fn flatten_directory(root: &PathBuf) {
    WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .for_each(|f| {
            let dest = root.join(f.file_name());
            if let Err(err) = fs::rename(f.path(), dest) {
                eprintln!("Error moving file: {}", err);
            }
        })
}

fn remove_directories(root: &PathBuf) -> Result<()> {
    for path in fs::read_dir(root)?
        .into_iter()
        .filter_map(|f| f.ok())
        .filter(|f| f.path().is_dir())
        .map(|f: fs::DirEntry| f.path())
    {
        if let Err(err) = remove_directories(&path) {
            eprintln!("Error removing directory: {}", err);
        }
        if let Err(err) = fs::remove_dir(path) {
            eprintln!("Error removing directory: {}", err);
        }
    }

    Ok(())
}
