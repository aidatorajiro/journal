pub mod subwindow {
    use std::borrow::Cow;

    use egui::{self, FontTweak, FontData, FontFamily};

    use bevy::{prelude::*, window::{WindowClosed, CreateWindow, PresentMode, WindowId}};
    use bevy_egui::EguiContext;
    use bevy_render::{MainWorld, render_graph::RenderGraph};
    use crate::{typedef::{state::*, component::*}, constants::constants::SECONDARY_EGUI_PASS};

    pub fn subwindow_ui_blank_page (mut egui_ctx: ResMut<EguiContext>, query: Query<&SubWindow, With<BlankPage>>) {
        for sw in query.iter() {
            let wid = match sw.window_id {None => continue, Some(a) => a};
            let ctx = match egui_ctx.try_ctx_for_window_mut(wid) {None => continue,Some(ctx) => ctx};
            
            egui::CentralPanel::default().show(ctx, |ui| {ui.label("Just Blank Page")});
        }
    }

    pub fn subwindow_ui_memo_field (mut egui_ctx: ResMut<EguiContext>, mut query: Query<&mut SubWindow, With<MemoField>>, mut global_state: ResMut<GameState>) {
        for mut sw in query.iter_mut() {
            let wid = match sw.window_id {None => continue, Some(a) => a};
            let ctx = match egui_ctx.try_ctx_for_window_mut(wid) {None => continue,Some(ctx) => ctx};

            egui::CentralPanel::default()
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    if ui.button("💾 Add Memo 💾").clicked() {
                        
                    }
                    ui.add_sized(ui.available_size(), egui::TextEdit::multiline(&mut global_state.textarea).margin(egui::Vec2{x:9.0, y:6.0}));
                });
                if sw.initialized == false {
                    println!("initializing second window....");

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
                    sw.initialized = true;
                }
            });
        }
    }

    pub fn subwindow_ui_set () -> SystemSet {
        SystemSet::new()
            .with_system(subwindow_ui_blank_page)
            .with_system(subwindow_ui_memo_field)
    }

    pub fn subwindow_event (
        query: Query<(Entity, &SubWindow)>,
        mut ev_close: EventReader<WindowClosed>,
        mut commands: Commands) {
        
        for ev in ev_close.iter() {
            for (ent, win) in query.iter() {
                if ev.id == win.window_id.unwrap() {
                    commands.entity(ent).despawn();
                    println!("triggered remove SubWindow {:?}", ev.id);
                }
            }
        }
    }

    pub fn subwindow_subapp_system (mut world: ResMut<MainWorld>, mut graph: ResMut<RenderGraph>) {        
        let mut binding_subwin = world.query::<&mut SubWindow>();
        
        let windowids = binding_subwin.iter_mut(&mut world).map(|mut x| {
            if x.window_id.is_none() {
                let wid = WindowId::new();
                x.window_id = Some(wid);
                
                bevy_egui::setup_pipeline(
                    &mut graph,
                    bevy_egui::RenderGraphConfig {
                        window_id: wid,
                        egui_pass: SECONDARY_EGUI_PASS,
                    },
                );
                Some(wid)
            } else {
                None
            }
        }).collect::<Vec<_>>();

        for wid_opt in windowids {
            if let Some(wid) = wid_opt {
                world.send_event::<CreateWindow>(CreateWindow {
                    id: wid,
                    descriptor: WindowDescriptor {
                        width: 800.,
                        height: 600.,
                        present_mode: PresentMode::AutoVsync,
                        title: "Second window".to_string(),
                        ..Default::default()
                    },
                });
                println!("make window id {:?}", wid);
            }
        }
    }
}