//! Main Program

pub mod constants;
pub mod typedef;
pub mod subwindow;

use constants::constants::*;
use subwindow::subwindow::*;
use typedef::state::*;
use typedef::event::*;

use bevy::{prelude::*, render::RenderApp};
use bevy_egui::EguiPlugin;
use bevy_render::{RenderStage};

use crate::typedef::component::SubWindow;
use crate::typedef::component::WindowType;

/// Main function
fn main() {
    let mut app = App::new();
    
    app.init_resource::<GameState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_startup_system(setup)
        .add_system(system_drag_and_drop)
        //.add_system(ui_second_window)
        //.add_system(second_window_event_handler)
        .add_system_to_stage(CoreStage::First, subwindow_event)
        .add_system(subwindow_ui);
    
    let mut render_app = app.sub_app_mut(RenderApp);
    //render_app.add_system_to_stage(RenderStage::Extract, second_window_subapp_system);
    render_app.add_system_to_stage(RenderStage::Extract, subwindow_subapp_system);

    app.run();
}

/// setup function for bevy
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 2d camera
    commands.spawn_bundle(Camera2dBundle::default());
}

/// Event listener for file drag and drop event.
fn system_drag_and_drop(
    mut dnd_ev: EventReader<FileDragAndDrop>,
    mut commands: Commands
) {
    for ev in dnd_ev.iter() {
        println!("{:?}", ev);
        match ev {
            FileDragAndDrop::DroppedFile { id, path_buf } => {
                commands.spawn().insert(SubWindow{window_type: WindowType::MemoField, ..default()});
            }
            FileDragAndDrop::HoveredFile { .. } => {},
            FileDragAndDrop::HoveredFileCancelled { .. } => {},
        }
    }
}
