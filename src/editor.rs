use std::hash::{DefaultHasher, Hash, Hasher};

use egui::{CentralPanel, ScrollArea, TextEdit};

pub(crate) struct Editor {
    pub(crate) content: String,
    hasher: DefaultHasher,
    previous_hash: u64,
}

impl Editor {
    pub(crate) fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let panel_response = CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                let response =
                    ui.add_sized(ui.available_size(), TextEdit::multiline(&mut self.content));

                self.hasher.write(self.content.as_bytes());
                let hash = self.hasher.finish();
                let hash_changed = hash != self.previous_hash;
                self.previous_hash = hash;

                if response.changed() && !self.content.is_empty() && hash_changed {
                    self.content = self.content.to_string();
                }
            });
        });
    }
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            content: String::new(),
        }
    }
}
