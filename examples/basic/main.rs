use flatten_directory::FlattenDirectory;

fn main() {
    FlattenDirectory::new("/tmp/test".into()).execute().unwrap();
}
