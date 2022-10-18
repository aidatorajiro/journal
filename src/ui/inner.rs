//! inner mechanisms for UI.
use bevy::prelude::*;

use crate::typedef::component::{MainCamera2D, MainCamera3D};

pub fn use_2d_camera (mut commands: Commands, q_2d: Query<Entity, With<MainCamera2D>>, q_3d: Query<Entity, With<MainCamera3D>>) {
    for x in q_3d.iter() {
        commands.entity(x).despawn_recursive()
    }
    if q_2d.is_empty() {
        commands.spawn_bundle(Camera2dBundle::default()).insert(MainCamera2D {});
    }
}

pub fn use_3d_camera (mut commands: Commands, q_2d: Query<Entity, With<MainCamera2D>>, q_3d: Query<Entity, With<MainCamera3D>>) {
    for x in q_2d.iter() {
        commands.entity(x).despawn_recursive()
    }
    if q_3d.is_empty() {
        commands.spawn_bundle(Camera3dBundle::default()).insert(MainCamera3D {});
    }
}
