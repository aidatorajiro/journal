//! Main Program

pub mod constants;
pub mod typedef;
pub mod subwindow;
pub mod assets;
pub mod utils;
pub mod journalmanage;

use assets::assets::RawData;
use assets::assets::RawDataLoader;

use journalmanage::systems::*;
use subwindow::systems::*;
use typedef::event::*;
use typedef::resource::*;

use bevy::{prelude::*, render::RenderApp};
use bevy_egui::EguiPlugin;
use bevy_render::{RenderStage};

/// Main function
fn main() {
    let mut app = App::new();
    
    app.init_resource::<GameState>()
        .add_event::<AddFragments>()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_asset::<RawData>()
        .init_asset_loader::<RawDataLoader>()
        .add_startup_system(setup)
        .add_system(system_drag_and_drop)
        .add_system(subwindow_event)
        .add_system_set(subwindow_ui_set())
        .add_system(handle_add_fragments);
    
    let render_app = app.sub_app_mut(RenderApp);
    render_app.add_system_to_stage(RenderStage::Extract, subwindow_subapp_system);

    app.run();
}

/// setup function for bevy
fn setup(mut commands: Commands) {
    // 2d camera
    commands.spawn_bundle(Camera2dBundle::default());
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