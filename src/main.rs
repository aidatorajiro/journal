//! Main Program

pub mod constants;
pub mod typedef;
pub mod subwindow;
pub mod assets;
pub mod utils;
pub mod journalmanage;
pub mod ui;

use assets::assets::RawData;
use assets::assets::RawDataLoader;

use bevy::app::AppExit;
use bevy::window::WindowClosed;
use bevy::winit::WinitSettings;
use journalmanage::systems::*;
use subwindow::systems::*;
use typedef::component::*;
use typedef::event::*;
use typedef::resource::*;
use constants::style::*;

use bevy::{prelude::*, render::RenderApp};
use bevy_egui::EguiPlugin;
use bevy::render::{RenderStage};
use typedef::state::*;
use ui::top::*;

/// Main function
fn main() {
    let mut app = App::new();
    
    app.init_resource::<GameGraph>()
        .add_state::<AppState>(AppState::Top)
        .insert_resource(WinitSettings::desktop_app())
        .add_event::<AddFragments>()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_asset::<RawData>()
        .init_asset_loader::<RawDataLoader>()
        .add_startup_system(setup)
        .add_system(system_drag_and_drop)
        // subwindows
        .add_system(subwindow_event)
        .add_system_set(subwindow_ui_set())
        .add_system(handle_add_fragments)
        .add_system(window_closed_handler)
        // Toppage System
        .add_startup_system(top_startup)
        .add_system_set(top_systems());
    
    let render_app = app.sub_app_mut(RenderApp);
    render_app.add_system_to_stage(RenderStage::Extract, subwindow_subapp_system);

    app.run();
}

/// setup function for bevy
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 2d camera
    commands.spawn_bundle(Camera2dBundle::default());

    //commands.spawn()
    //    .insert(SubWindow { initialized: false, window_id: None })
    //    .insert(MemoField { textarea: "Hello".to_string() });
}

fn window_closed_handler(mut ev: EventReader<WindowClosed>, mut quit: EventWriter<AppExit>) {
    for e in ev.iter() {
        if e.id.is_primary() {
            quit.send(AppExit);
        }
    }
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
