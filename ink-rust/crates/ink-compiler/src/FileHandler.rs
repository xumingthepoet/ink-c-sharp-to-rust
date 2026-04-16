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
