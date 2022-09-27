//! Main Program

mod constants;
mod typedef;
mod second_window;

use constants::constants::*;
use typedef::state::*;
use typedef::event::*;
use second_window::second_window::*;

use std::{fs::File, borrow::Cow, collections::HashMap};
use serde::{Deserialize, Serialize};
use rmp_serde::{Serializer, Deserializer};
use bevy::{prelude::*, text::Text2dBounds, render::{render_graph::RenderGraph, once_cell::sync::Lazy, RenderApp, camera::RenderTarget}, window::{WindowId, CreateWindow, PresentMode}};
use bevy_egui::{EguiContext, EguiPlugin};
use egui::{self, FontFamily, FontData, FontTweak, FontDefinitions};
use bevy_render::{RenderStage};

/// Main function
fn main() {
    let mut app = App::new();
    
    app.init_resource::<GameState>()
        .add_event::<OpenSecondWindow>()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_startup_system(setup)
        .add_system(system_drag_and_drop)
        .add_system(ui_second_window)
        .add_system(second_window_event_handler);
    
    let mut render_app = app.sub_app_mut(RenderApp);
    render_app.add_system_to_stage(RenderStage::Extract, second_window_subapp_system);

    app.run();
}

/// setup function for bevy
fn setup(mut global_state: ResMut<GameState>, mut commands: Commands, asset_server: Res<AssetServer>) {
    // 2d camera
    commands.spawn_bundle(Camera2dBundle::default());
}

/// Event listener for file drag and drop event.
fn system_drag_and_drop(
    mut dnd_ev: EventReader<FileDragAndDrop>,
    mut evw: EventWriter<OpenSecondWindow>
) {
    for ev in dnd_ev.iter() {
        println!("{:?}", ev);
        match ev {
            FileDragAndDrop::DroppedFile { id, path_buf } => {
                evw.send(OpenSecondWindow {});
            }
            FileDragAndDrop::HoveredFile { id, path_buf } => {
            }
            FileDragAndDrop::HoveredFileCancelled { id } => {
            }
        }
    }
}
