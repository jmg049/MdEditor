use egui_file::FileDialog;

pub(crate) struct DialogManager {
    file_dialog: Option<FileDialog>,
}

impl Default for DialogManager {
    fn default() -> Self {
        Self { file_dialog: None }
    }
}
