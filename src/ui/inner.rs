//! utility functions for UI.

use bevy::prelude::*;

use crate::typedef::component::{MainCamera2D, MainCamera3D};

/// Delete all 3d cameras, and then spawn new 2d camera.
pub fn use_default_2d_camera (mut commands: Commands, q_2d: Query<Entity, With<MainCamera2D>>, q_3d: Query<Entity, With<MainCamera3D>>) {
    for x in q_3d.iter() {
        commands.entity(x).despawn_recursive()
    }
    if q_2d.is_empty() {
        commands.spawn_bundle(Camera2dBundle::default()).insert(MainCamera2D {});
    }
}

/// Delete all cameras.
pub fn delete_all_camera (mut commands: Commands, q_2d: Query<Entity, With<MainCamera2D>>, q_3d: Query<Entity, With<MainCamera3D>>) {
    for x in q_2d.iter() {
        commands.entity(x).despawn_recursive()
    }
    for x in q_3d.iter() {
        commands.entity(x).despawn_recursive()
    }
}