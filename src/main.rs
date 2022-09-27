//! Main Program

mod constants;
mod typedef;
mod second_window;

use crate::constants::constants::*;
use crate::typedef::typedef::*;
use crate::second_window::second_window::*;

use std::{fs::File, borrow::Cow, collections::HashMap};
use serde::{Deserialize, Serialize};
use rmp_serde::{Serializer, Deserializer};
use bevy::{prelude::*, text::Text2dBounds, render::{render_graph::RenderGraph, once_cell::sync::Lazy, RenderApp, camera::RenderTarget}, window::{WindowId, CreateWindow, PresentMode}};
use bevy_egui::{EguiContext, EguiPlugin};
use egui::{self, FontFamily, FontData, FontTweak, FontDefinitions};

/// Main function
fn main() {
    let mut app = App::new();
    
    app.init_resource::<GameState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_startup_system(setup)
        .add_system(system_drag_and_drop)
        .add_system(ui_second_window);
    
    let render_app = app.sub_app_mut(RenderApp);

    let mut graph = render_app.world.get_resource_mut::<RenderGraph>().unwrap();

    bevy_egui::setup_pipeline(
        &mut graph,
        bevy_egui::RenderGraphConfig {
            window_id: *SECOND_WINDOW_ID,
            egui_pass: SECONDARY_EGUI_PASS,
        },
    );

    app.run();
}

/// setup function for bevy
fn setup(mut create_window_events: EventWriter<CreateWindow>, mut global_state: ResMut<GameState>, mut commands: Commands, asset_server: Res<AssetServer>) {
    // 2d camera
    commands.spawn_bundle(Camera2dBundle::default());

    create_window_events.send(CreateWindow {
        id: *SECOND_WINDOW_ID,
        descriptor: WindowDescriptor {
            width: 800.,
            height: 600.,
            present_mode: PresentMode::AutoVsync,
            title: "Second window".to_string(),
            ..Default::default()
        },
    });
}

/// Event listener for file drag and drop event.
fn system_drag_and_drop(
    mut dnd_ev: EventReader<FileDragAndDrop>
) {
    for ev in dnd_ev.iter() {
        println!("{:?}", ev);
        match ev {
            FileDragAndDrop::DroppedFile { id, path_buf } => {
            }
            FileDragAndDrop::HoveredFile { id, path_buf } => {
            }
            FileDragAndDrop::HoveredFileCancelled { id } => {
            }
        }
    }
}
