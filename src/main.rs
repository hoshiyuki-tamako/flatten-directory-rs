use anyhow::Result;
use clap::Parser;
use flatten_directory::FlattenDirectory;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(required = true)]
    path: PathBuf,
}

fn main() -> Result<()> {
    let args: Args = Args::parse();
    FlattenDirectory::new(args.path).execute()?;
    Ok(())
}
