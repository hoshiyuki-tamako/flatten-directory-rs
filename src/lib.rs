use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct FlattenDirectory {
    root: PathBuf,
}

impl FlattenDirectory {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    pub fn execute(&self) -> Result<(), std::io::Error> {
        self.flatten()?;
        self.remove_directories(&self.root)?;

        Ok(())
    }

    fn flatten(&self) -> Result<(), std::io::Error> {
        let files = WalkDir::new(&self.root)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file());

        for f in files {
            let mut dest = self.root.join(f.file_name());
            // handle duplicated names
            if dest.exists() {
                let mut c = 0u128;
                let p = f.path();
                let filename = p.file_stem().unwrap_or_default().to_string_lossy();
                let ext = p.extension().unwrap_or_default().to_string_lossy();
                let dot = if ext == "" { "" } else { "." };
                loop {
                    dest = self.root.join(format!("{filename}_{c}{dot}{ext}"));
                    if !dest.exists() {
                        break;
                    }
                    c += 1;
                }
            }

            fs::rename(f.path(), dest)?;
        }

        Ok(())
    }

    fn remove_directories(&self, root: &PathBuf) -> Result<(), std::io::Error> {
        for path in fs::read_dir(root)?
            .into_iter()
            .filter_map(|f| f.ok())
            .filter(|f| f.path().is_dir())
            .map(|f: fs::DirEntry| f.path())
        {
            self.remove_directories(&path)?;
            fs::remove_dir(path)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fake::{Fake, Faker};

    #[test]
    fn test_default() {
        let path = Faker.fake::<PathBuf>();
        let fd = FlattenDirectory::new(path.clone());
        assert_eq!(fd.root, path);
    }
}
