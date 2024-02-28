use egui::{FontId, TextStyle};

#[derive(Debug, Clone, Copy)]
pub struct FontConfig {
    pub(crate) regular_size: f32,
    pub(crate) heading_size: f32,
    pub(crate) monospace_size: f32,
    pub(crate) button_size: f32,
    pub(crate) small_size: f32,
    pub(crate) as_vec: Vec<(TextStyle, FontId)>,
}

impl FontConfig {}

impl Default for FontConfig {
    fn default() -> Self {
        Self {
            regular_size: 16.0,
            heading_size: 24.0,
            monospace_size: 16.0,
            button_size: 16.0,
            small_size: 12.0,
            as_vec: vec![
                (
                    TextStyle::Heading,
                    FontId::new(24.0, egui::FontFamily::Proportional),
                ),
                (
                    TextStyle::Body,
                    FontId::new(16.0, egui::FontFamily::Proportional),
                ),
                (
                    TextStyle::Monospace,
                    FontId::new(16.0, egui::FontFamily::Monospace),
                ),
                (
                    TextStyle::Button,
                    FontId::new(16.0, egui::FontFamily::Proportional),
                ),
                (
                    TextStyle::Small,
                    FontId::new(12.0, egui::FontFamily::Proportional),
                ),
            ],
        }
    }
}

impl Into<Vec<(TextStyle, FontId)>> for FontConfig {
    fn into(self) -> Vec<(TextStyle, FontId)> {
        vec![
            (
                TextStyle::Heading,
                FontId::new(self.heading_size, egui::FontFamily::Proportional),
            ),
            (
                TextStyle::Body,
                FontId::new(self.regular_size, egui::FontFamily::Proportional),
            ),
            (
                TextStyle::Monospace,
                FontId::new(self.monospace_size, egui::FontFamily::Monospace),
            ),
            (
                TextStyle::Button,
                FontId::new(self.button_size, egui::FontFamily::Proportional),
            ),
            (
                TextStyle::Small,
                FontId::new(self.small_size, egui::FontFamily::Proportional),
            ),
        ]
    }
}
