//! Main Program

pub mod constants;
pub mod typedef;
pub mod subwindow;
pub mod assets;
pub mod utils;
pub mod journalmanage;
pub mod ui;

use assets::loader::RawData;
use assets::loader::RawDataLoader;

use bevy::app::AppExit;
use bevy::window::PresentMode;
use bevy::window::WindowClosed;
use bevy::winit::WinitSettings;
use journalmanage::systems::*;
use subwindow::systems::*;
use typedef::event::*;
use typedef::resource::*;

use bevy::{prelude::*, render::RenderApp};
use bevy_egui::EguiPlugin;
use bevy::render::{RenderStage};
use typedef::state::*;
use ui::explore::*;
use ui::router::ui_manage_systems;
use ui::migrate::migrate_systems_enter;
use ui::migrate::migrate_systems_exit;
use ui::migrate::migrate_systems_update;
use ui::newpage::*;
use ui::top::*;

/// Main function
pub fn run_the_journal() {
    let mut app = App::new();
    
    app.insert_resource(WindowDescriptor {
        title: "! Bevy Journal ! (c) 2022 Torajiro Aida".to_string(),
        width: 800.,
        height: 800.,
        present_mode: PresentMode::AutoVsync,
        resizable: false,
        ..default()
    })
        .init_resource::<GameGraph>()
        .add_state::<AppState>(AppState::TopPage)
        .insert_resource(WinitSettings::desktop_app())
        .add_event::<AddFragments>()
        .add_event::<SyncFragments>()
        .add_event::<SyncFragmentsDone>()
        .add_event::<JumpToNewPage>()
        .add_event::<JumpToExplore>()
        .add_event::<JumpToLinear>()
        .add_event::<JumpToMigrate>()
        .add_event::<JumpToTop>()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_asset::<RawData>()
        .init_asset_loader::<RawDataLoader>()
        .add_startup_system(setup)
        .add_system(system_drag_and_drop)
        .add_system_set(ui_manage_systems())
        // journal manage
        .add_system(handle_sync_fragments)
        // subwindows
        .add_system(subwindow_event)
        .add_system_set(subwindow_ui_set())
        .add_system(window_closed_handler)
        // Toppage System
        .add_system_set(top_systems_enter())
        .add_system_set(top_systems_exit())
        .add_system_set(top_systems_update())
        // Newpage system
        .add_system_set(newpage_systems_enter())
        .add_system_set(newpage_systems_exit())
        .add_system_set(newpage_systems_update())
        // Migrate system
        .add_system_set(migrate_systems_enter())
        .add_system_set(migrate_systems_exit())
        .add_system_set(migrate_systems_update())
        // Explore system
        .add_system_set(explore_systems_enter())
        .add_system_set(explore_systems_exit())
        .add_system_set(explore_systems_update());
    
    let render_app = app.sub_app_mut(RenderApp);
    render_app.add_system_to_stage(RenderStage::Extract, subwindow_subapp_system);

    app.run();
}

/// setup function for bevy
fn setup() { }

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
