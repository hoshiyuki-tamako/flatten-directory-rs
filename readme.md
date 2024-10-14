# Flatten directory

1. move all files from sub-folders to target directory

2. then remove all directories recursively

## Install

```bash
cargo install flatten-directory
```

## Usage

### Command Line

```bash
flatten-directory .
```

```bash
flatten-directory "/mnt/d/download"
```

## Use as library

### Basic

```rs
use flatten_directory::FlattenDirectory;

fn main() {
    FlattenDirectory::new("/tmp/test".into()).execute().unwrap();
}
```
