//! UI definitions for explore
use std::f32::consts::PI;

use bevy::{prelude::*, utils::HashMap, render::camera::Projection};
use bevy_mod_picking::{PickingCameraBundle, PickableBundle, Selection, PickingSystem};
use petgraph::graph::NodeIndex;
use rand::random;
use crate::{typedef::{state::AppState, event::JumpToExplore, resource::{ExploreState, GameGraph}, component::{ExploreContents, ExploreCube, Fragment, FragmentContents}}, utils::graph::{make_force_graph_nodes, make_force_graph_edges}, constants::style::{EXPLORE_CUBE_CLICKED, EXPLORE_CUBE_HOVERED, EXPLORE_CUBE_NONE, EXPLORE_CUBE_SELECTED}};

use fdg_sim::{ForceGraph, ForceGraphHelper, Simulation, SimulationParameters, Dimensions, force::{fruchterman_reingold_weighted}};

use super::inner::delete_all_camera;

pub fn explore_systems_enter () -> SystemSet {
    return SystemSet::on_enter(AppState::Explore).with_system(delete_all_camera).with_system(explore_enter);
}

pub fn explore_systems_exit () -> SystemSet {
    return SystemSet::on_exit(AppState::Explore).with_system(explore_exit);
}

pub fn explore_systems_update () -> SystemSet {
    return SystemSet::on_update(AppState::Explore)
        .with_system(explore_update_graph)
        .with_system(explore_update_interaction.after(PickingSystem::Highlighting)); // to avoid 
}

fn explore_enter (
    mut com: Commands,
    mut ev_explore: EventReader<JumpToExplore>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    game_graph: Res<GameGraph>,
) {
    let mut page_state = ExploreState::default();

    for _ in ev_explore.iter() {
        // do some processing of page arguments
    }

    com.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 25.0).looking_at(Vec3::ZERO, Vec3::X),
        projection: Projection::Perspective(PerspectiveProjection {
            fov: PI / 2.0,
            ..default()
        }),
        ..default()
    }).insert_bundle(PickingCameraBundle::default());

    let mut force_graph: ForceGraph<Entity, f32> = ForceGraph::default();

    let mut map_ent_idx: HashMap<Entity, NodeIndex> = HashMap::new();

    // ↓ START force-graph generation, combining many kinds of graphs into one force-graph to be visualized ↓

    make_force_graph_nodes(&mut force_graph, &game_graph.neighbor_graph, &mut map_ent_idx);

    make_force_graph_nodes(&mut force_graph, &game_graph.history_graph, &mut map_ent_idx);

    make_force_graph_edges(&mut force_graph, &game_graph.neighbor_graph, &map_ent_idx, |_, _, _, _| 1.0);

    make_force_graph_edges(&mut force_graph, &game_graph.history_graph, &map_ent_idx, |_, _, _, _| 1.0);

    // ↑ END force-graph generation ↑

    for e in force_graph.node_weights() {
        com.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.3 })),
            material: materials.add(EXPLORE_CUBE_NONE.into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        }).insert(ExploreCube {
            force_graph_index: map_ent_idx.get(&e.data).unwrap().clone(),
            fragment_id: e.data.clone()
        }).insert_bundle(PickableBundle { ..default()}).insert(ExploreContents {});
    }

    let mut params = SimulationParameters::default();
    params.dimensions = Dimensions::Three;
    params.node_start_size = 1.0;
    params.set_force(fruchterman_reingold_weighted(0.25, 0.975)); // tips: increasing `scale` will diffuses graph, making nodes more equally distributed.
    let simulation = Simulation::from_graph(force_graph, params);

    page_state.simulation = Some(simulation);

    /*
    com.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 80.0),
        ..default()
    }).insert(ExploreContents {});*/

    com.insert_resource::<ExploreState>(page_state);
}

fn explore_exit (q: Query<Entity, With<ExploreContents>>, mut com: Commands) {
    com.remove_resource::<ExploreState>();

    for i in q.iter() {
        com.entity(i).despawn_recursive();
    }
}

fn explore_update_graph (mut q_cube: Query<(&mut Transform, &ExploreCube)>, mut page: ResMut<ExploreState>) {
    let sim = page.simulation.as_mut().unwrap();
    sim.update(0.035);
    
    for (mut t, ec) in q_cube.iter_mut() {
        let node_data = sim.get_graph().node_weight(ec.force_graph_index).unwrap();
        t.translation.x = node_data.location.x;
        t.translation.y = node_data.location.y;
        t.translation.z = node_data.location.z;

        // Add random numbers to t.rotation, but make sure not exceeding certain value!
        let apply_rand = |z: &mut f32| {*z +=  (random::<f32>() - 0.5)/10.0; *z = z.max(-0.5).min(0.5)};
        apply_rand(&mut t.rotation.x);
        apply_rand(&mut t.rotation.y);
        apply_rand(&mut t.rotation.z);
    }
}

fn explore_update_interaction(
    mut q_cube: Query<(&Interaction, &ExploreCube, &Selection, &Handle<StandardMaterial>), Changed<Interaction>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut q_fragment: Query<&Fragment>
) {
    for (interaction, cube, selection, mat_handle) in q_cube.iter() {
        match interaction {
            Interaction::Clicked => {
                let m = materials.get_mut(mat_handle).unwrap();
                m.emissive = EXPLORE_CUBE_CLICKED;
            },
            Interaction::Hovered => {
                let m = materials.get_mut(mat_handle).unwrap();
                m.emissive = EXPLORE_CUBE_HOVERED;
            },
            Interaction::None => {
                let m = materials.get_mut(mat_handle).unwrap();
                m.base_color = EXPLORE_CUBE_NONE;
            }
        };
        
        if selection.selected() {
            let m = materials.get_mut(mat_handle).unwrap();
            m.emissive = EXPLORE_CUBE_SELECTED;
        }

        if let Ok(f) = q_fragment.get(cube.fragment_id) {
            match f.contents.clone() {
                FragmentContents::TextData { data } => {},
                FragmentContents::Code { data, language } => todo!(),
                FragmentContents::URL { data } => todo!(),
                FragmentContents::Image { data } => todo!(),
            }
        }
    }
}