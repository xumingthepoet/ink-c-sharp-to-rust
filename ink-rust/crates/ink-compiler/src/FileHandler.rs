// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/compiler/FileHandler.cs

use std::io;
use std::path::PathBuf;

pub trait IFileHandler {
    fn ResolveInkFilename(&self, includeName: &str) -> io::Result<String>;
    fn LoadInkFileContents(&self, fullFilename: &str) -> io::Result<String>;
}

#[derive(Clone, Debug, Default)]
pub struct DefaultFileHandler;

impl IFileHandler for DefaultFileHandler {
    fn ResolveInkFilename(&self, includeName: &str) -> io::Result<String> {
        let working_dir = std::env::current_dir()?;
        let full_root_ink_path: PathBuf = working_dir.join(includeName);
        Ok(full_root_ink_path.to_string_lossy().into_owned())
    }

    fn LoadInkFileContents(&self, fullFilename: &str) -> io::Result<String> {
        std::fs::read_to_string(fullFilename)
    }
}

#[cfg(test)]
mod tests {
    use super::{DefaultFileHandler, IFileHandler};
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn resolves_relative_filenames_from_current_dir() {
        let handler = DefaultFileHandler;
        let resolved = handler.ResolveInkFilename("story.ink").unwrap();
        let current_dir = std::env::current_dir().unwrap();
        assert!(resolved.starts_with(current_dir.to_string_lossy().as_ref()));
        assert!(resolved.ends_with("story.ink"));
    }

    #[test]
    fn loads_file_contents() {
        let handler = DefaultFileHandler;
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!("ink_file_handler_{unique}.txt"));
        fs::write(&path, "hello ink").unwrap();

        let contents = handler
            .LoadInkFileContents(path.to_string_lossy().as_ref())
            .unwrap();
        assert_eq!(contents, "hello ink");

        let _ = fs::remove_file(path);
    }
}
