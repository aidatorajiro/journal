pub mod utils {
    use std::{time::{SystemTime, UNIX_EPOCH}, borrow::Cow};

    use egui::{Ui, FontTweak, FontFamily, FontData};

    /// Returns current timestamp in second.
    pub fn create_timestamp() -> u64 {
        return SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    /// Set font.
    pub fn set_default_font(ui: &mut Ui) {
        let mut fonts = egui::text::FontDefinitions::default();

        fonts.font_data.insert("notosans".to_string(), FontData {
            font: Cow::from(include_bytes!("../assets/NotoSansJP-Thin.otf").to_vec()),
            index: 0,
            tweak: FontTweak {
                scale: 2.0,
                y_offset_factor: 0.0,
                y_offset: -6.0,
            },
        });
        
        fonts.font_data.insert("notoemoji".to_string(), FontData {
            font: Cow::from(include_bytes!("../assets/NotoEmoji-VariableFont_wght.ttf").to_vec()),
            index: 0,
            tweak: FontTweak {
                scale: 2.0,
                y_offset_factor: 0.0,
                y_offset: -6.0,
            },
        });

        fonts.families.insert(FontFamily::Proportional, vec!["notosans".to_string(), "notoemoji".to_string()]);
        ui.ctx().set_fonts(fonts.clone());
        
    }
}