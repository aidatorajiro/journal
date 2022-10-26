//! Main Program

pub mod constants;
pub mod typedef;
pub mod subwindow;
pub mod assets;
pub mod utils;
pub mod journalmanage;
pub mod ui;
pub mod migration;
pub mod tests;

use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use assets::loader::RawData;
use assets::loader::RawDataLoader;

use bevy::app::AppExit;
use bevy::reflect::{TypeRegistry, ReflectSerialize, ReflectDeserialize};
use bevy::tasks::IoTaskPool;
use bevy::window::PresentMode;
use bevy::window::WindowClosed;
use bevy::winit::WinitSettings;
use journalmanage::systems::*;
use subwindow::systems::*;
use typedef::component::*;
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
use utils::utils::create_timestamp;

use crate::constants::style::STATE_FILE;

/// Main function
pub fn run_the_journal() {
    let mut app = App::new();

    app
        .insert_resource(WindowDescriptor {
            title: "! Bevy Journal ! (c) 2022 Torajiro Aida".to_string(),
            width: 800.,
            height: 800.,
            present_mode: PresentMode::AutoVsync,
            resizable: false,
            ..default()
        })
        .register_type::<Entity>()
        .register_type::<FragmentContents>()
        .register_type_data::<FragmentContents, ReflectSerialize>()
        .register_type_data::<FragmentContents, ReflectDeserialize>()
        .register_type::<String>()
        .register_type_data::<String, ReflectSerialize>()
        .register_type_data::<String, ReflectDeserialize>()
        .register_type::<Fragment>()
        .register_type::<EntityList>()
        .register_type::<Entry>()
        .register_type::<Tag>()
        .register_type::<TagEvent>()
        .register_type::<TagEventAction>()
        .register_type::<GameGraphDummy>()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_state::<AppState>(AppState::LoadSaveData)
        .insert_resource(WinitSettings::desktop_app())
        .add_event::<AddFragments>()
        .add_event::<SyncFragments>()
        .add_event::<SyncFragmentsDone>()
        .add_event::<JumpToNewPage>()
        .add_event::<JumpToExplore>()
        .add_event::<JumpToLinear>()
        .add_event::<JumpToMigrate>()
        .add_event::<JumpToTop>()
        .init_resource::<StartupManagement>()
        .add_asset::<RawData>()
        .init_resource::<GameGraph>()
        .init_asset_loader::<RawDataLoader>()

        // Root systems
        .add_startup_system(load_scene_system.exclusive_system())
        .add_system(load_graph_system)
        .add_system(save_scene_system.exclusive_system())
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

/// Load scene
fn load_scene_system(world: &mut World) {
    println!("Checking ron file...");
    if Path::new(STATE_FILE).exists() { // TODO: maybe use IoTaskPool?
        println!("Loading ron file...");
        fs::copy(Path::new(STATE_FILE), Path::new(STATE_FILE).with_extension("ron.".to_string() + &create_timestamp().to_string())).unwrap();
        let asset_server = world.resource_mut::<AssetServer>();
        let handle: Handle<DynamicScene> = asset_server.load(Path::new("..").join(STATE_FILE));
        //let mut spawner =  SceneSpawner::default();
        //let res = spawner.spawn_sync(world, handle);
        //println!("{:?}", res);
        let id = world.spawn().insert_bundle(DynamicSceneBundle {
            scene: handle,
            ..default()
        }).id();
        println!("Root entity id: {:?}", id);
    } else {
        let id = world.spawn().id();
        println!("Root entity id: {:?}", id);
        
        let mut startup = world.resource_mut::<StartupManagement>();
        startup.state_file_nonexistent = true;

        let mut stat = world.resource_mut::<State<AppState>>();
        stat.overwrite_set(AppState::TopPage).unwrap();
    }
    let mut startup = world.resource_mut::<StartupManagement>();
    startup.state_file_checked = true;
}

/// Load scene, additional steps for the graph
fn load_graph_system(
    mut stat: ResMut<State<AppState>>,
    mut r: ResMut<GameGraph>,
    mut startup: ResMut<StartupManagement>,
    mut commands: Commands,
    q: Query<(Entity, &GameGraphDummy)>,
    q_test: Query<Entity, Without<GameGraphDummy>>,
) {
    let (e, d) = match q.get_single() {Ok(x) => x, _ => return};

    println!("Loading dummy state...");
    
    *r = GameGraph {
        neighbor_graph: ron::from_str(&d.neighbor_graph).unwrap(),
        neighbor_graph_ids: ron::from_str(&d.neighbor_graph_ids).unwrap(),
        history_graph: ron::from_str(&d.history_graph).unwrap(),
        history_graph_ids: ron::from_str(&d.history_graph_ids).unwrap()
    };

    commands.entity(e).despawn();

    startup.load_graph_done = true;

    println!("{:?}", q_test.iter().collect::<Vec<_>>());
    
    stat.overwrite_set(AppState::TopPage).unwrap();
}

/// Save scene
fn save_scene_system(world: &mut World) {

    if world.resource::<State<AppState>>().current().clone() == AppState::LoadSaveData { return; }

    if !world.is_resource_changed::<GameGraph>() { return; }

    println!("Saving state...");

    let graph = world.get_resource::<GameGraph>().unwrap();

    let dummy = GameGraphDummy {
        neighbor_graph: ron::to_string(&graph.neighbor_graph).unwrap(),
        neighbor_graph_ids: ron::to_string(&graph.neighbor_graph_ids).unwrap(),
        history_graph: ron::to_string(&graph.history_graph).unwrap(),
        history_graph_ids: ron::to_string(&graph.history_graph_ids).unwrap()
    };

    world.spawn().insert(dummy);

    if world.despawn(Entity::from_bits(0)) {
        println!("Entity 0 removed");
    }

    let type_registry = TypeRegistry::default();
    type_registry.write().register::<Entity>();
    type_registry.write().register_type_data::<Entity, ReflectSerialize>();
    type_registry.write().register_type_data::<Entity, ReflectDeserialize>();
    type_registry.write().register::<FragmentContents>();
    type_registry.write().register_type_data::<FragmentContents, ReflectSerialize>();
    type_registry.write().register_type_data::<FragmentContents, ReflectDeserialize>();
    type_registry.write().register::<String>();
    type_registry.write().register_type_data::<String, ReflectSerialize>();
    type_registry.write().register_type_data::<String, ReflectDeserialize>();
    type_registry.write().register::<Fragment>();
    type_registry.write().register::<EntityList>();
    type_registry.write().register::<Entry>();
    type_registry.write().register::<Tag>();
    type_registry.write().register::<TagEvent>();
    type_registry.write().register::<TagEventAction>();
    type_registry.write().register::<GameGraphDummy>();

    let scene = DynamicScene::from_world(&world, &type_registry);
    let serialized_scene = match scene.serialize_ron(&type_registry) {Ok(x) => x, Err(x) => {println!("{:?}", x); return}};

    println!("Success! Cleaning...");

    let mut q = world.query::<(Entity, &GameGraphDummy)>();
    world.despawn(q.single(world).0);
    
    #[cfg(not(target_arch = "wasm32"))]
    IoTaskPool::get()
        .spawn(async move {
            File::create(STATE_FILE)
                .and_then(|mut file| file.write(serialized_scene.as_bytes()))
                .expect("Error while saving state!");
        })
        .detach();
    
    #[cfg(target_arch = "wasm32")]
    todo!("TODO: put here localStorage or communication with server or something");
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
