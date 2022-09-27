pub mod second_window {
    use std::borrow::Cow;

    use bevy::prelude::{ResMut, Local};
    use bevy_egui::{EguiContext, egui::{FontTweak, FontData, FontFamily}};

    use crate::{typedef::typedef::*, constants::constants::*};

    /// Second window handler.
    pub fn ui_second_window(mut egui_ctx: ResMut<EguiContext>, mut window_state: Local<GameWindowState>, mut global_state: ResMut<GameState>) {
        let ctx = match egui_ctx.try_ctx_for_window_mut(*SECOND_WINDOW_ID) {
            Some(ctx) => ctx,
            None => return,
        };
        
        egui::CentralPanel::default()
            .show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.button("Hello");
                ui.add_sized(ui.available_size(), egui::TextEdit::multiline(&mut global_state.textarea));
            });
            if (window_state.initialized == false) {
                println!("initializing second window....");

                let mut fonts = egui::text::FontDefinitions::default();
                fonts.font_data.insert("myfont".to_string(), FontData {
                    font: Cow::from(include_bytes!("../assets/NotoSansJP-Thin.otf").to_vec()),
                    index: 0,
                    tweak: FontTweak {
                        scale: 2.0,
                        y_offset_factor: 0.0,
                        y_offset: -6.0,
                    },
                });
                fonts.families.insert(FontFamily::Proportional, vec!["myfont".to_string()]);
                ui.ctx().set_fonts(fonts.clone());
                window_state.initialized = true;
            }
        });
    }

}