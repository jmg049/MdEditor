#![feature(let_chains)]
mod command;
mod error;

use eframe;
use egui::{self, FontDefinitions};
use std::{
    borrow::Cow,
    ffi::OsStr,
    fmt::Debug,
    path::{Path, PathBuf},
};

use egui_commonmark::CommonMarkViewer;
use egui_file::FileDialog;
use egui_toast::{Toast, ToastKind, ToastOptions, Toasts};
use error::MdResult;

use crate::command::Command;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1920.0, 1080.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Mardown Editor",
        options,
        Box::new(|cc| Box::<MdEdit>::default()),
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MdEditState {
    Editor,
    Command,
    Files,
    Invalid,
}

const STATE_ORDERING: [MdEditState; 3] = [
    MdEditState::Editor,
    MdEditState::Command,
    MdEditState::Files,
];

struct MdEdit {
    current_file: Option<PathBuf>,
    current_dir: Option<PathBuf>,
    cache: egui_commonmark::CommonMarkCache,
    current_content: String,
    content_changed: bool,
    file_dialogue: Option<FileDialog>,
    command_input: String,
    state: usize,
}

impl Default for MdEdit {
    fn default() -> Self {
        Self {
            current_file: None,
            current_dir: Some(PathBuf::from(std::env::current_dir().unwrap())),
            cache: egui_commonmark::CommonMarkCache::default(),
            current_content: String::from(""),
            content_changed: false,
            file_dialogue: None,
            command_input: String::from(""),
            state: 0,
        }
    }
}

impl MdEdit {
    pub(crate) fn set_current_directory(&mut self, dir: &str) {
        self.current_dir = Some(PathBuf::from(dir));
    }

    fn save_current_file<P: AsRef<Path> + Debug>(
        file: P,
        content: &str,
        toasts: &mut Toasts,
    ) -> std::io::Result<()> {
        std::fs::write(&file, content)?;
        dbg!("File saved to {:?}", &file);
        toasts.add(Toast {
            text: format!("File saved to {:?}", &file).into(),
            kind: ToastKind::Success,
            options: ToastOptions::default()
                .duration_in_seconds(2.5)
                .show_progress(true),
        });
        Ok(())
    }

    fn files_in_current_dir(&self) -> Vec<PathBuf> {
        let mut files = vec![];

        if let Some(dir) = &self.current_dir {
            if let Ok(entries) = std::fs::read_dir(dir) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        if entry.file_type().unwrap().is_dir() {
                            files.push(entry.path());
                        } else if entry.file_type().unwrap().is_file()
                            && entry.path().extension() == Some(OsStr::new("md"))
                        {
                            files.push(entry.path());
                        }
                    }
                }
            }
        } else {
            if let Ok(entries) = std::fs::read_dir(".") {
                for entry in entries {
                    if let Ok(entry) = entry {
                        if entry.file_type().unwrap().is_dir() {
                            files.push(entry.path());
                        } else if entry.file_type().unwrap().is_file()
                            && entry.path().extension() == Some(OsStr::new("md"))
                        {
                            files.push(entry.path());
                        }
                    }
                }
            }
        }
        files.push("..".into());
        files.sort();
        files
    }

    fn toolbar(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) -> MdResult<()> {
        Ok(())
    }

    fn command_panel(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
    ) -> MdResult<egui::Response> {
        Ok(egui::TopBottomPanel::bottom("Console")
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Command: ");

                    let response = ui.add_sized(
                        ui.available_size(),
                        egui::TextEdit::singleline(&mut self.command_input),
                    );

                    if response.lost_focus()
                        && response.ctx.input(|i| i.key_pressed(egui::Key::Enter))
                    {
                        self.handle_command();
                        self.command_input.clear();
                    }
                });
            })
            .response)
    }

    fn handle_command(&mut self) {
        dbg!("Command entered: {:?}", &self.command_input);
        let command = Command::from(self.command_input.as_str());
        command.execute(self);
    }

    fn file_explorer(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
        width: f32,
        toasts: &mut Toasts,
    ) -> MdResult<egui::Response> {
        Ok(egui::SidePanel::left("Files")
            .exact_width(width)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.vertical(|ui| {
                        let files = self.files_in_current_dir();
                        for file in files {
                            let file_name = match file.file_name() {
                                Some(name) => name.to_string_lossy(),
                                None => {
                                    if file == Path::new("..") {
                                        Cow::from("..")
                                    } else {
                                        continue;
                                    }
                                }
                            };

                            let file_name_label = ui.selectable_label(false, file_name.clone());
                            if file_name_label.clicked() {
                                if file_name == ".." {
                                    let parent = match &self.current_dir {
                                        Some(dir) => dir.parent(),
                                        None => {
                                            self.current_dir.as_ref().map(|d| d.parent()).flatten()
                                        }
                                    };
                                    match parent {
                                        Some(parent) => {
                                            self.current_dir = Some(parent.to_path_buf());
                                        }
                                        None => (),
                                    }
                                } else if file.is_dir() {
                                    self.current_dir = Some(file);
                                } else if !self.content_changed {
                                    self.current_content =
                                        std::fs::read_to_string(&file).unwrap_or_default();
                                    self.current_file = Some(file);
                                } else {
                                    dbg!("Content changed, prompt user to save");
                                    toasts.add(Toast {
                                        text: "Please save file before opening another".into(),
                                        kind: ToastKind::Error,
                                        options: ToastOptions::default()
                                            .duration_in_seconds(2.5)
                                            .show_progress(true),
                                    });
                                }
                            }
                        }
                    });
                });
            })
            .response)
    }

    fn editor(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) -> MdResult<()> {
        Ok(())
    }

    fn preview(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) -> MdResult<()> {
        Ok(())
    }
}

impl eframe::App for MdEdit {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        catppuccin_egui::set_theme(ctx, catppuccin_egui::MACCHIATO);
        let window_size = ctx.input(|i| i.viewport().outer_rect).unwrap();

        ctx.input(|i| {
            let ctrl_modifier = i.modifiers.command_only();
            let right_arrow = i.key_pressed(egui::Key::ArrowRight);
            let left_arrow = i.key_pressed(egui::Key::ArrowLeft);

            match (ctrl_modifier, right_arrow) {
                (true, true) => {
                    dbg!("Ctrl + >");
                    self.state = (self.state + STATE_ORDERING.len() - 1) % STATE_ORDERING.len();
                }
                _ => (),
            }

            match (ctrl_modifier, left_arrow) {
                (true, true) => {
                    dbg!("Ctrl + <");
                    self.state = (self.state + 1) % STATE_ORDERING.len();
                }
                _ => (),
            }
        });

        let left_panel_width = window_size.width() * 0.1;
        let right_panel_width = window_size.width() * 0.45;

        let mut toasts = Toasts::new()
            .anchor(egui::Align2::RIGHT_BOTTOM, (-10.0, -10.0)) // 10 units from the bottom right corner
            .direction(egui::Direction::BottomUp);

        let command_response = match self.command_panel(ctx, _frame) {
            Ok(res) => res,
            Err(e) => {
                panic!(
                    "Error encountered while creating the command panel: {:?}",
                    e
                );
            }
        };

        let toolbar_response = egui::TopBottomPanel::top("Toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let mut do_save = false;
                if ui.button("Save").clicked() {
                    match &self.current_file {
                        Some(_) => {
                            self.file_dialogue = None;
                            do_save = true;
                        }
                        None => {
                            let filter = Box::new({
                                let ext = Some(OsStr::new("md"));
                                move |p: &Path| p.extension() == ext || p.is_dir()
                            });
                            let mut dialog = FileDialog::save_file(self.current_file.clone())
                                .show_files_filter(filter);
                            dialog.open();
                            self.file_dialogue = Some(dialog);
                            do_save = true;
                        }
                    }
                }

                if ui.button("Save As").clicked() {
                    let filter = Box::new({
                        let ext = Some(OsStr::new("md"));
                        move |p: &Path| p.extension() == ext || p.is_dir()
                    });
                    let mut dialog =
                        FileDialog::save_file(self.current_file.clone()).show_files_filter(filter);
                    dialog.open();
                    self.file_dialogue = Some(dialog);
                    do_save = true;
                }

                if ui.button("New").clicked() {
                    // for now save current file if it exists
                    match &self.current_file {
                        Some(_) => {
                            match MdEdit::save_current_file(
                                &self.current_file.as_ref().unwrap(),
                                &self.current_content,
                                &mut toasts,
                            ) {
                                Ok(()) => {
                                    self.content_changed = false;
                                }
                                Err(e) => {
                                    dbg!(e);
                                }
                            }
                        }
                        None => (),
                    }

                    self.current_file = None;
                    self.current_content = String::from("");
                }

                // If we didn't open the dialogue, we must already have a file name, so just save it
                if let Some(dialog) = &mut self.file_dialogue {
                    if dialog.show(ctx).selected() {
                        if let Some(file) = dialog.path() {
                            match MdEdit::save_current_file(
                                file,
                                &self.current_content,
                                &mut toasts,
                            ) {
                                Ok(()) => {
                                    self.content_changed = false;
                                }
                                Err(e) => {
                                    dbg!(e);
                                }
                            }
                        }
                    }
                } else if do_save {
                    match &self.current_file {
                        Some(file) => {
                            match MdEdit::save_current_file(
                                file,
                                &self.current_content,
                                &mut toasts,
                            ) {
                                Ok(()) => {
                                    self.content_changed = false;
                                }
                                Err(e) => {
                                    dbg!(e);
                                }
                            }
                        }
                        None => (),
                    }
                }
            });
        });

        let files_resposne = match self.file_explorer(ctx, _frame, left_panel_width, &mut toasts) {
            Ok(res) => res,
            Err(e) => {
                panic!(
                    "Error encountered while creating the file explorer panel: {:?}",
                    e
                );
            }
        };

        let preview_response = egui::SidePanel::right("Preview")
            .resizable(true)
            .exact_width(right_panel_width)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    CommonMarkViewer::new("Preview").show(
                        ui,
                        &mut self.cache,
                        &self.current_content,
                    );
                });
            })
            .response;

        let editor_response = egui::CentralPanel::default()
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    let response = ui.add_sized(
                        ui.available_size(),
                        egui::TextEdit::multiline(&mut self.current_content),
                    );

                    if response.changed() {
                        dbg!("Content changed");
                        self.content_changed = true;
                    }
                });
            })
            .response;

        toasts.show(ctx);
    }
}
