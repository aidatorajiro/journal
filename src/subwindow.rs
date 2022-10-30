pub mod systems {
    //! "second window" management. This program makes subwindows into Entities, making it easier to manage subwindows, their inner states, and their creation/destruction mechanism.
    
    use std::borrow::Cow;

    use egui::{self, FontTweak, FontData, FontFamily, TextEdit, style::Margin};

    use bevy::{prelude::*, window::{WindowClosed, CreateWindow, PresentMode, WindowId}};
    use bevy_egui::EguiContext;
    use bevy::render::{MainWorld, render_graph::RenderGraph};
    use crate::{typedef::{component::*, event::*}, constants::window::SECONDARY_EGUI_PASS};

    /// Blank window UI definition.
    pub fn subwindow_ui_blank_page (mut egui_ctx: ResMut<EguiContext>, query: Query<&SubWindow, With<BlankPage>>) {
        for sw in query.iter() {
            let wid = match sw.window_id {None => continue, Some(a) => a};
            let ctx = match egui_ctx.try_ctx_for_window_mut(wid) {None => continue,Some(ctx) => ctx};
            
            egui::CentralPanel::default().show(ctx, |ui| {ui.label("Just Blank Page")});
        }
    }

    /// Returns a set of Subwindow UIs that will be added to the App object.
    pub fn subwindow_ui_set () -> SystemSet {
        SystemSet::new()
            .with_system(subwindow_ui_blank_page)
    }

    /// A event handler for WindowClosed event. Detects window close and delete the entity corresponding with the window.
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

    /// A system with stage beby_render::RenderStage::Extract. Create windowid when a window entity is created in the "main" world. Then, setup pipeline to enable the rendering, and send a CreateWindow event.
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
                        title: "Sub window".to_string(),
                        ..Default::default()
                    },
                });
                println!("make window id {:?}", wid);
            }
        }
    }
}