use crate::dialog::DialogManager;
use crate::editor::Editor;
use crate::files::FilesManager;
use crate::fonts::FontConfig;
use crate::{command::CommandManager, hotkeys::KeyManager};
use egui::{Key, Response, TextStyle};
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use egui_toast::{Toast, Toasts};
use epaint::FontId;
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use crate::error::MdResult;

pub(crate) struct MdEditor<'s> {
    files_manager: FilesManager,
    preview_cache: CommonMarkCache,
    editor: Editor,
    dialog_manager: DialogManager,
    command_manager: CommandManager,
    key_manager: KeyManager<'s>,
    notifications: Toasts,
    font_config: FontConfig,
}

impl<'s> MdEditor<'s> {
    pub(crate) fn new(dir: PathBuf, include_hidden_dirs: bool) -> Self {
        Self {
            files_manager: FilesManager::new(dir, include_hidden_dirs),
            preview_cache: CommonMarkCache::default(),
            editor: Editor::default(),
            dialog_manager: DialogManager::default(),
            command_manager: CommandManager::default(),
            key_manager: KeyManager::default(),
            notifications: Toasts::new()
                .anchor(egui::Align2::RIGHT_BOTTOM, (0.0, 0.0))
                .direction(egui::Direction::BottomUp),
            font_config: FontConfig::default(),
        }
    }

    fn setup_fonts(&self, ctx: &egui::Context) {
        let mut style = (*ctx.style()).clone();
        let font_config = &self.font_config;

        font_config.as_vec.iter().for_each(|(s, font_id)| {
            style.text_styles.insert(s.clone(), font_id.clone());
        });
        ctx.set_style(style);
    }

    #[inline(always)]
    fn setup_theme(&self, ctx: &egui::Context) {
        catppuccin_egui::set_theme(ctx, catppuccin_egui::MACCHIATO);
    }

    fn preview(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
        panel_width: f32,
    ) -> MdResult<Response> {
        let side_panel = egui::SidePanel::right("Preview")
            .resizable(false)
            .exact_width(panel_width)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    CommonMarkViewer::new("Preview").show(
                        ui,
                        &mut self.preview_cache,
                        &self.editor.content,
                    );
                });
            })
            .response;
        Ok(side_panel)
    }
}

impl<'s> eframe::App for MdEditor<'s> {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.command_manager
            .update(ctx, frame, &mut self.editor, &mut self.files_manager);
        self.files_manager.update(ctx, frame);
        self.editor.update(ctx, frame);
        self.key_manager.handle(ctx);

        self.setup_theme(ctx);
        self.setup_fonts(ctx);

        let window_size = ctx.input(|i| i.viewport().outer_rect).unwrap();

        let files_width = window_size.width() * 0.1;
        let preview_width = window_size.width() * 0.45;
        // editor takes up the rest of the space

        let command_response = match self.command_manager.show(ctx, frame) {
            Ok(response) => response,
            Err(e) => {
                panic!(
                    "Error encountered while creating the command panel: {:?}",
                    e
                );
            }
        };

        let files_response =
            match self
                .files_manager
                .show(ctx, frame, files_width, &mut self.notifications)
            {
                Ok(response) => response,
                Err(e) => {
                    panic!("Error encountered while creating the files panel: {:?}", e);
                }
            };

        let preview_response = match self.preview(ctx, frame, preview_width) {
            Ok(res) => res,
            Err(e) => {
                panic!(
                    "Error encountered while creating the preview panel: {:?}",
                    e
                );
            }
        };

        let editor_response = match self.editor.show(ctx, frame) {
            Ok(res) => res,
            Err(e) => {
                panic!("Error encountered while creating the editor panel: {:?}", e);
            }
        };
        // notifications
        self.notifications.show(ctx);
    }
}
