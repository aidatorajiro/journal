//! UI definitions for explore
use bevy::prelude::*;
use rand::random;
use crate::typedef::{state::AppState, event::JumpToExplore, resource::ExploreState, component::{ExploreContents, ExploreCube}};

use super::inner::{use_3d_camera, delete_all_camera};

pub fn explore_systems_enter () -> SystemSet {
    return SystemSet::on_enter(AppState::Explore).with_system(delete_all_camera).with_system(explore_enter.after(use_3d_camera));
}

pub fn explore_systems_exit () -> SystemSet {
    return SystemSet::on_exit(AppState::Explore).with_system(explore_exit);
}

pub fn explore_systems_update () -> SystemSet {
    return SystemSet::on_update(AppState::Explore).with_system(explore_update);
}

fn explore_enter (
    mut com: Commands,
    mut ev_explore: EventReader<JumpToExplore>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for ev in ev_explore.iter() {
        com.insert_resource::<ExploreState>(ExploreState {});
    }

    com.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 25.0).looking_at(Vec3::ZERO, Vec3::X),
        ..default()
    });

    com.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.4 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    }).insert(ExploreContents {}).insert(ExploreCube {});

    com.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    }).insert(ExploreContents {});

    com.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 80.0),
        ..default()
    }).insert(ExploreContents {});
}

fn explore_exit (q: Query<Entity, With<ExploreContents>>, mut com: Commands) {
    com.remove_resource::<ExploreState>();

    for i in q.iter() {
        com.entity(i).despawn_recursive();
    }
}

fn explore_update (mut q_cube: Query<&mut Transform, With<ExploreCube>>) {
    println!("updupd");
    for mut t in q_cube.iter_mut() {
        println!("translating");
        let applyrand = |z: &mut f32| {*z +=  (random::<f32>() - 0.5)/10.0; *z = z.max(-0.5).min(0.5)};
        applyrand(&mut t.rotation.x);
        applyrand(&mut t.rotation.y);
        applyrand(&mut t.rotation.z);
    }
}