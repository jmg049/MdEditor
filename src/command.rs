use crate::{
    core::MdEditor,
    editor::{self, Editor},
    files::FilesManager,
    MdEdit,
};

pub(crate) struct CommandManager {
    history: Vec<Command>,
}

impl CommandManager {
    pub(crate) fn update(
        &mut self,
        ctx: &egui::Context,
        frame: &mut eframe::Frame,
        editor: &mut Editor,
        file_manager: &mut FilesManager,
    ) {
        todo!()
    }
}

impl Default for CommandManager {
    fn default() -> Self {
        Self {
            history: Vec::new(),
        }
    }
}

pub enum Command {
    Save(String),
    Open(String),
    New,
    SetDirectory(String),
    Invalid,
}

impl Command {
    pub fn execute(&self, md_edit: &mut MdEdit) {
        match self {
            Command::Save(file) => {
                dbg!("Saving file: {}", file);
            }
            Command::Open(file) => {
                dbg!("Opening file: {}", file);
            }
            Command::New => {
                dbg!("Creating new file");
            }
            Command::SetDirectory(dir) => {
                dbg!("Setting directory: {}", dir);

                let dir = match dir.as_bytes()[0] {
                    b'~' => {
                        dbg!("Squigly path");
                        shellexpand::tilde(dir).to_string()
                    }
                    _ => {
                        dbg!("Relative path");
                        dir.to_owned()
                    }
                };

                md_edit.set_current_directory(&dir);
            }
            Command::Invalid => {
                dbg!("Invalid command");
            }
        }
    }
}

impl From<&str> for Command {
    fn from(value: &str) -> Self {
        let split_command = value.split_whitespace().collect::<Vec<&str>>();
        match split_command[0] {
            "save" => Command::Save(split_command[1].to_string()),
            "open" => Command::Open(split_command[1].to_string()),
            "new" => Command::New,
            "setdir" => Command::SetDirectory(split_command[1].to_string()),
            _ => Command::Invalid,
        }
    }
}

impl From<String> for Command {
    fn from(value: String) -> Self {
        Command::from(value.as_str())
    }
}
