//! Main program dayo

use std::fs::File;

use bevy::{prelude::*, text::Text2dBounds};
use bevy_egui::{egui, EguiContext, EguiPlugin};

#[derive(Component)]
struct MyComponent;

#[derive(Default)]
struct MyState {
    textarea: String,
}

fn main() {
    App::new()
        .init_resource::<MyState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_startup_system(setup)
        .add_system(system_drag_and_drop)
        .add_system(ui_example)
        .add_system(update_text)
        .run();
}

fn setup(mut my_state: ResMut<MyState>, mut commands: Commands, asset_server: Res<AssetServer>) {
    // 2d camera
    commands.spawn_bundle(Camera2dBundle::default());
    
    commands.spawn_bundle(Text2dBundle {
        text: Text::from_section("", TextStyle {
            font: asset_server.load("NotoSansJP-Thin.otf"),
            font_size: 60.0,
            color: Color::WHITE,
        }).with_alignment(TextAlignment::CENTER),
        transform: Transform::from_xyz(
            0.0,
            0.0,
            1.0,
        ),
        ..default()
    }).insert(MyComponent);
}

fn update_text(asset_server: Res<AssetServer>, mut my_state: ResMut<MyState>, mut query: Query<&mut Text, With<MyComponent>>) {
    for mut txt in &mut query {
        txt.apply(&Text::from_section(my_state.textarea.clone(), TextStyle {
            font: asset_server.load("NotoSansJP-Thin.otf"),
            font_size: 60.0,
            color: Color::WHITE,
        }).with_alignment(TextAlignment::CENTER));
    }
}

fn ui_example(mut my_state: ResMut<MyState>, mut egui_context: ResMut<EguiContext>) {
    egui::Window::new("Hello").show(egui_context.ctx_mut(), |ui| {
        ui.label("world");
        ui.text_edit_multiline(&mut my_state.textarea)
    });
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

