pub mod second_window {
    use std::borrow::Cow;

    use bevy::{prelude::{ResMut, Local, EventReader, EventWriter}, window::{WindowDescriptor, CreateWindow, PresentMode, WindowClosed, WindowId}};
    use bevy_egui::{EguiContext, egui::{FontTweak, FontData, FontFamily}};
    use bevy_render::{MainWorld, render_graph::RenderGraph};

    use crate::{typedef::{state::*, event::*}, constants::constants::*};

    /// Second window handler.
    pub fn ui_second_window(mut egui_ctx: ResMut<EguiContext>, mut global_state: ResMut<GameState>) {
        let second_window_id = match global_state.second_window.id {
            None => return,
            Some(a) => a
        };

        let ctx = match egui_ctx.try_ctx_for_window_mut(second_window_id) {
            Some(ctx) => ctx,
            None => return,
        };
        
        egui::CentralPanel::default()
            .show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.button("Hello");
                ui.add_sized(ui.available_size(), egui::TextEdit::multiline(&mut global_state.textarea));
            });
            if (global_state.second_window.initialized == false) {
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
                global_state.second_window.initialized = true;
            }
        });
    }

    pub fn second_window_event_handler(
            mut global_state: ResMut<GameState>, 
            mut create_window_events: EventWriter<CreateWindow>, 
            ev_secwin: EventReader<OpenSecondWindow>, 
            mut ev_close: EventReader<WindowClosed>) {

        if (!ev_secwin.is_empty() && !global_state.second_window.opened) {
            global_state.second_window.opened = true;
            global_state.second_window.initialized = false;
        }

        for e in ev_close.iter() {
            if (e.id == global_state.second_window.id.unwrap()) {
                global_state.second_window.opened = false;
            }
        }
    }

    pub fn second_window_subapp_system (mut world: ResMut<MainWorld>, mut graph: ResMut<RenderGraph>) {
        let mut gs = world.get_resource_mut::<GameState>().unwrap();

        if gs.second_window.id.is_some() && !gs.second_window.opened {
            graph.remove_node(SECONDARY_EGUI_PASS);
            gs.second_window.id = None;
        }
        
        if gs.second_window.id.is_none() && gs.second_window.opened {
            let wid = WindowId::new();
            gs.second_window.id = Some(wid);
            
            bevy_egui::setup_pipeline(
                &mut graph,
                bevy_egui::RenderGraphConfig {
                    window_id: wid,
                    egui_pass: SECONDARY_EGUI_PASS,
                },
            );

            (move || world)().send_event::<CreateWindow>(CreateWindow {
                id: wid,
                descriptor: WindowDescriptor {
                    width: 800.,
                    height: 600.,
                    present_mode: PresentMode::AutoVsync,
                    title: "Second window".to_string(),
                    ..Default::default()
                },
            });

            println!("Heyyy!");
        }
    }

}