//! UI definitions for explore
use bevy::{prelude::*, utils::HashMap};
use petgraph::graph::NodeIndex;
use rand::random;
use crate::typedef::{state::AppState, event::JumpToExplore, resource::{ExploreState, GameGraph}, component::{ExploreContents, ExploreCube}};

use fdg_sim::{ForceGraph, ForceGraphHelper, Simulation, SimulationParameters, Dimensions, force::{fruchterman_reingold_weighted}};

use super::inner::delete_all_camera;

pub fn explore_systems_enter () -> SystemSet {
    return SystemSet::on_enter(AppState::Explore).with_system(delete_all_camera).with_system(explore_enter);
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
    game_graph: Res<GameGraph>,
) {
    let mut page_state = ExploreState::default();

    for _ in ev_explore.iter() {
        // do some processing of page arguments
    }

    com.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 25.0).looking_at(Vec3::ZERO, Vec3::X),
        ..default()
    });

    let mut force_graph: ForceGraph<Entity, f32> = ForceGraph::default();

    let mut map_ent_idx: HashMap<Entity, NodeIndex> = HashMap::new();

    // ↓ START force-graph generation, combining many kinds of graphs into one force-graph to be visualized ↓

    for e in game_graph.neighbor_graph.node_weights() {
        let idx = force_graph.add_force_node(format!("{:?}", e.clone()), e.clone());
        map_ent_idx.insert(e.clone(), idx);
    }

    for i in game_graph.neighbor_graph.edge_indices() {
        let (a_idx, b_idx) = game_graph.neighbor_graph.edge_endpoints(i).unwrap();
        let a_wgt = game_graph.neighbor_graph.node_weight(a_idx).unwrap();
        let b_wgt = game_graph.neighbor_graph.node_weight(b_idx).unwrap();
        force_graph.add_edge(map_ent_idx.get(a_wgt).unwrap().clone(), map_ent_idx.get(b_wgt).unwrap().clone(), 1.0);
    }

    // ↑ END force-graph generation ↑

    let mut params = SimulationParameters::default();
    params.dimensions = Dimensions::Three;
    params.node_start_size = 1.0;
    params.set_force(fruchterman_reingold_weighted(0.3, 0.975)); // tips: enlarging `scale` will diffuses graph, making nodes more equally distributed.
    page_state.simulation = Some(Simulation::from_graph(force_graph, params));

    for e in game_graph.neighbor_graph.node_weights() {
        com.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.3 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        }).insert(ExploreContents {}).insert(ExploreCube {
            force_graph_index: map_ent_idx.get(&e).unwrap().clone(),
            fragment_id: e.clone()
        });
    }

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

    com.insert_resource::<ExploreState>(page_state);
}

fn explore_exit (q: Query<Entity, With<ExploreContents>>, mut com: Commands) {
    com.remove_resource::<ExploreState>();

    for i in q.iter() {
        com.entity(i).despawn_recursive();
    }
}

fn explore_update (mut q_cube: Query<(&mut Transform, &ExploreCube)>, mut page: ResMut<ExploreState>) {
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