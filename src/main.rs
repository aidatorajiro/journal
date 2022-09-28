//! Main Program

pub mod constants;
pub mod typedef;
pub mod subwindow;
pub mod assets;

use std::fs::File;
use std::io::prelude::*;

use assets::assets::RawData;
use assets::assets::RawDataLoader;
use bevy_render::texture::CompressedImageFormats;
use bevy_render::texture::ImageType;
use subwindow::subwindow::*;
use typedef::component::*;
use typedef::state::*;

use bevy::{prelude::*, render::RenderApp};
use bevy_egui::EguiPlugin;
use bevy_render::{RenderStage};

/// Main function
fn main() {
    let mut app = App::new();
    
    app.init_resource::<GameState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_asset::<RawData>()
        .init_asset_loader::<RawDataLoader>()
        .add_startup_system(setup)
        .add_system(system_drag_and_drop)
        .add_system(subwindow_event)
        .add_system_set(subwindow_ui_set());
    
    let render_app = app.sub_app_mut(RenderApp);
    render_app.add_system_to_stage(RenderStage::Extract, subwindow_subapp_system);

    app.run();
}

/// setup function for bevy
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 2d camera
    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle((SubWindow::default(), MemoField));

    commands.spawn()
        .insert(SwashText {})
        .insert(asset_server.load::<RawData, _>("Nabla-Regular-VariableFont_EDPT,EHLT.ttf"));
}

/// Event listener for file drag and drop event.
fn system_drag_and_drop(
    mut dnd_ev: EventReader<FileDragAndDrop>
) {
    for ev in dnd_ev.iter() {
        println!("{:?}", ev);
        match ev {
            FileDragAndDrop::DroppedFile { .. } => {}
            FileDragAndDrop::HoveredFile { .. } => {},
            FileDragAndDrop::HoveredFileCancelled { .. } => {},
        }
    }
}