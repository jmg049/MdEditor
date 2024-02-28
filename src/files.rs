use std::path::PathBuf;

use crate::{core::MdEditor, editor::Editor, error::MdResult};

#[derive(Debug)]
pub(crate) struct FilesManager {
    pub(crate) current_dir: PathBuf, // will always be given a directory
    pub(crate) current_file: Option<PathBuf>,
    pub(crate) files: Vec<PathBuf>, // only gets updated when the user clicks a new file/directory
    pub(crate) include_hidden_dirs: bool,
}

impl FilesManager {
    pub(crate) fn new(current_dir: PathBuf, include_hidden_dirs: bool) -> Self {
        let files =
            files_in_dir(&current_dir.to_str().unwrap_or("~"), include_hidden_dirs).unwrap();
        Self {
            current_dir,
            current_file: None,
            files,
            include_hidden_dirs: false,
        }
    }

    pub(crate) fn update_current_dir(&mut self, dir: PathBuf) -> MdResult<()> {
        self.current_dir = dir;
        let files = files_in_dir(
            &self.current_dir.to_str().unwrap_or("."),
            self.include_hidden_dirs,
        )?;
        self.files = files;
        Ok(())
    }

    pub(crate) fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        todo!()
    }
}

pub(crate) fn files_in_dir<P: AsRef<str>>(
    dir: P,
    include_hidden_dirs: bool,
) -> MdResult<Vec<PathBuf>> {
    let dir = PathBuf::from(shellexpand::tilde(&dir.as_ref()).to_string());
    dbg!("Path exists: {:?}", dir.exists());
    Ok(std::fs::read_dir(dir)?
        .into_iter()
        .filter_map(Result::ok)
        .filter(|d| {
            let is_markdown = d.path().extension() == Some(OsStr::new("md"));
            let is_hidden = d.file_name().to_string_lossy().starts_with('~');
            let is_dir = d.file_type().map(|t| t.is_dir()).unwrap_or(false);
            is_markdown || (is_dir && (include_hidden_dirs || !is_hidden))
        })
        .map(|d| d.path())
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_files_in_dir() {
        let dir = Path::new("~/code");
        let files = files_in_dir(dir.to_string_lossy(), false).unwrap();
        println!("{:?}", files);

        let dir = Path::new("~/");
        let files = files_in_dir(dir.to_string_lossy(), false).unwrap();
        println!("{:?}", files);
    }
}
