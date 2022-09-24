//! Main program dayo

use std::fs::File;

use bevy::{prelude::*, text::Text2dBounds};

#[derive(Component)]
struct MyComponent;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(system_drag_and_drop)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("NotoSansJP-Thin.otf");

    let text_style = TextStyle {
        font,
        font_size: 60.0,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment::CENTER;

    // 2d camera
    commands.spawn_bundle(Camera2dBundle::default());
    
    commands.spawn_bundle(Text2dBundle {
        text: Text::from_section("あああああああ", text_style.clone()).with_alignment(text_alignment),
        transform: Transform::from_xyz(
            0.0,
            0.0,
            1.0,
        ),
        ..default()
    }).insert(MyComponent);
}

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
