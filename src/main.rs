use anyhow::Result;
use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};
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
            let mut dest = root.join(f.file_name());
            // handle duplicated names
            if dest.exists() {
                let mut c = 0usize;
                loop {
                    let p = Path::new(f.file_name());
                    let filename = p.file_stem().unwrap().to_str().unwrap();
                    let ext = p.extension().unwrap().to_str().unwrap();
                    dest = root.join(format!("{filename}_{c}.{ext}"));
                    if !dest.exists() {
                        break;
                    }
                    c += 1;
                }
            }

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
