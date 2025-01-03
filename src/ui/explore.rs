//! UI definitions for explore.
//! In this page, the player can explore the graph database, which represents how each pieces of texts are modified (the history database) or connected (the neighborhood database).
//! It uses fdg-sim to calculate position of each node of the graph.

use std::f32::consts::PI;

use bevy::{prelude::*, utils::{HashMap, HashSet}, render::camera::Projection, ui::FocusPolicy};
use bevy_mod_picking::{prelude::PickSelection, PickableBundle};
use petgraph::graph::NodeIndex;
use rand::random;
use crate::{typedef::{state::AppState, event::{JumpToExplore, JumpToNewPage, JumpToTop}, resource::{ExploreState, GameGraph}, component::{ExploreContents, ExploreCube, Fragment, FragmentContents, ExploreFragmentText, EntityList, Entry, ExploreButton}}, utils::{graph::{make_force_graph_nodes, make_force_graph_edges}, basic::chunk_string}, constants::style::{EXPLORE_CUBE_CLICKED, EXPLORE_CUBE_HOVERED, EXPLORE_CUBE_NONE, EXPLORE_CUBE_SELECTED, TOPBTN_TEXT_COLOR, EXPLORE_TEXT_COLOR, EXPLORE_CUBE_SIZE, EXPLORE_NORMAL, TOPBTN_IMG_OVERLAY, EXPLORE_CLICK, EXPLORE_HOVER}};

use fdg_sim::{ForceGraph, ForceGraphHelper, Simulation, SimulationParameters, Dimensions, force::{fruchterman_reingold_weighted}};

use super::inner::delete_all_camera;

pub fn explore_enter (
    mut com: Commands,
    mut ev_explore: EventReader<JumpToExplore>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    game_graph: Res<GameGraph>,
    q_entry: Query<(Entity, &EntityList), With<Entry>>
) {
    let mut page_state = ExploreState::default();

    for _ in ev_explore.read() {
        // TODO: do some processing of page arguments
    }

    //let mut transform = Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::X, Vec3::Z);
    //transform.rotate_local_x(2.0*PI*rand::random::<f32>());
    //transform.rotate_local_y(2.0*PI*rand::random::<f32>());

    let transform = Transform::from_xyz(0.0, 0.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y);

    com.spawn(Camera3dBundle {
        transform,
        projection: Projection::Perspective(PerspectiveProjection {
            fov: PI / 1.5,
            ..default()
        }),
        ..default()
    }).insert(ExploreContents {});

    let mut force_graph: ForceGraph<Entity, f32> = ForceGraph::default();

    let mut map_ent_idx: HashMap<Entity, NodeIndex> = HashMap::new();

    // ↓ START force-graph generation, combining many kinds of graphs into one force-graph to be visualized ↓

    make_force_graph_nodes(&mut force_graph, &game_graph.neighbor_graph, &mut map_ent_idx);

    make_force_graph_nodes(&mut force_graph, &game_graph.history_graph, &mut map_ent_idx);

    make_force_graph_edges(&mut force_graph, &game_graph.neighbor_graph, &map_ent_idx, |_, _, _, _| 1.0);

    make_force_graph_edges(&mut force_graph, &game_graph.history_graph, &map_ent_idx, |_, _, _, _| 1.0);

    for (entry_id, l) in q_entry.iter() {
        for fragment_id in &l.entities {
            force_graph.add_edge(map_ent_idx.get(&entry_id).unwrap().clone(), map_ent_idx.get(fragment_id).unwrap().clone(), 1.0);
        }
    }

    // ↑ END force-graph generation ↑

    let handle = materials.add(EXPLORE_CUBE_NONE);

    for e in force_graph.node_weights() {
        com.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid { half_size: EXPLORE_CUBE_SIZE })),
            material: handle.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        }).insert(ExploreCube {
            force_graph_index: map_ent_idx.get(&e.data).unwrap().clone(),
            entity_id: e.data.clone()
        }).insert(PickableBundle { ..default()}).insert(ExploreContents {});
    }

    let mut params = SimulationParameters::default();
    params.dimensions = Dimensions::Three;
    params.node_start_size = 1.0;
    params.set_force(fruchterman_reingold_weighted(0.25, 0.975)); // tips: increasing `scale` will diffuses graph, making nodes more equally distributed.
    let simulation = Simulation::from_graph(force_graph, params);

    page_state.simulation = Some(simulation);

    let mut txt = TextBundle::from_section(
        "hello".to_string(),
        TextStyle {
            font: asset_server.load("NotoSansJP-Bold.otf"),
            font_size: 40.0,
            color: EXPLORE_TEXT_COLOR,
        }
    );

    txt.style.position_type = PositionType::Absolute;

    com.spawn(txt).insert(ExploreFragmentText {}).insert(ExploreContents {});;

    com.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(5.0),
            justify_content: JustifyContent::FlexEnd,
            align_items: AlignItems::FlexEnd,
            align_content: AlignContent::FlexEnd,
            flex_wrap: FlexWrap::Wrap,
            flex_direction: FlexDirection::Row,
            left: Val::Percent(0.0),
            top: Val::Percent(0.0),
            position_type: PositionType::Absolute,
            ..default()
        },
        background_color: Color::NONE.into(),
        ..default()
    }).with_children(|parent|{
        let base_w = 20.0;
        let base_h = 100.0;

        let tags = [ExploreButton::Return, ExploreButton::Merge];

        for tag in tags {
            parent.spawn(ButtonBundle {
                style: Style {
                    width: Val::Percent(base_w),
                    height: Val::Percent(base_h),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: EXPLORE_NORMAL.into(),
                ..default()
            }).with_children(|parent| {

                let image = match tag {
                    ExploreButton::Return => asset_server.load("explore.png").into(),
                    ExploreButton::Merge => asset_server.load("explore.png").into(),
                };

                parent.spawn(ImageBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        left: Val::Percent(0.0),
                        top: Val::Percent(0.0),
                        position_type: PositionType::Absolute,
                        
                        ..default()
                    },
                    background_color: TOPBTN_IMG_OVERLAY.into(),
                    focus_policy: FocusPolicy::Pass,
                    image,
                    ..default()
                });

                parent.spawn(TextBundle::from_section(
                    match tag {
                        ExploreButton::Return => "Return",
                        ExploreButton::Merge => "Merge"
                    }, 
                    TextStyle {
                        font: asset_server.load("NotoSansJP-Bold.otf"),
                        font_size: 40.0,
                        color: TOPBTN_TEXT_COLOR,
                    }
                ));
            }).insert(tag);
        }
    }).insert(ExploreContents {});

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

pub fn explore_exit (q: Query<Entity, With<ExploreContents>>, mut com: Commands) {
    com.remove_resource::<ExploreState>();

    for i in q.iter() {
        com.entity(i).despawn_recursive();
    }
}

pub fn explore_update_buttons(mut interaction_query: Query<
    (&Interaction, &ExploreButton, &mut BackgroundColor),
    Changed<Interaction>,
>,
mut ev_np: EventWriter<JumpToNewPage>,
mut ev_tp: EventWriter<JumpToTop>,
page: Res<ExploreState>,
game: Res<GameGraph>,
mut q_fragment: Query<&Fragment>,
mut q_entry: Query<&Entry, With<EntityList>>,
q_entity_list: Query<&EntityList>) {
    for (inter, btn_attr, mut color) in interaction_query.iter_mut() {
        match *inter {
            Interaction::Pressed => {
                *color = EXPLORE_CLICK.into();
                match btn_attr {
                    ExploreButton::Return => {
                        ev_tp.send(JumpToTop {});
                    },
                    ExploreButton::Merge => {
                        let vec_entity: HashSet<Entity> = page.selections.iter().map(|e|{
                            if let Ok(_) = q_fragment.get(e.clone()) {
                                game.fragment_to_entry.get(e).unwrap().iter().max_by(|a, b| {
                                    q_entity_list.get(**a).unwrap().timestamp.cmp(
                                        &q_entity_list.get(**b).unwrap().timestamp
                                    )
                                }).map(|x|x.clone())
                            } else if let Ok(_) = q_entry.get(e.clone()) {
                                Some(e.clone())
                            } else {
                                None
                            }
                        }).flatten().collect();
                        ev_np.send(JumpToNewPage { entry_ids: vec_entity.into_iter().collect() });
                    }
                };
            },
            Interaction::Hovered => {
                *color = EXPLORE_HOVER.into();
            },
            Interaction::None => {
                *color = EXPLORE_NORMAL.into();
            },
        }
    }
}

pub fn explore_update_graph (mut q_cube: Query<(&mut Transform, &ExploreCube)>, mut page: ResMut<ExploreState>) {
    if page.selections.is_empty() {
        let sim = page.simulation.as_mut().unwrap();
        sim.update(0.035);
        
        for (mut t, ec) in q_cube.iter_mut() {
            let node_data = sim.get_graph().node_weight(ec.force_graph_index).unwrap();
            t.translation.x = node_data.location.x;
            t.translation.y = node_data.location.y;
            t.translation.z = node_data.location.z;

            // Add random numbers to t.rotation, but make sure not exceeding certain value!
            //let apply_rand = |z: &mut f32| {*z +=  (random::<f32>() - 0.5)/10.0; *z = z.max(-0.5).min(0.5)};
            //apply_rand(&mut t.rotation.x);
            //apply_rand(&mut t.rotation.y);
            //apply_rand(&mut t.rotation.z);
        }
    }
}

pub fn explore_update_interaction(
    mut q_cube: Query<(&Interaction, &ExploreCube, &PickSelection)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut q_fragment: Query<&Fragment>,
    mut q_entry: Query<&EntityList, With<Entry>>,
    mut q_text: Query<&mut Text, With<ExploreFragmentText>>,
    mut page: ResMut<ExploreState>
) {

    page.hover_id = None;
    page.selections.clear();
    

    for (interaction, cube, selection) in q_cube.iter() {
        match interaction {
            Interaction::Pressed => {
                //let m = materials.get_mut(mat_handle).unwrap();
                //m.emissive = EXPLORE_CUBE_CLICKED;
            },
            Interaction::Hovered => {
                //let m = materials.get_mut(mat_handle).unwrap();
                //m.emissive = EXPLORE_CUBE_HOVERED;
                page.hover_id = Some(cube.entity_id);
            },
            Interaction::None => {
                //let m = materials.get_mut(mat_handle).unwrap();
                //m.base_color = EXPLORE_CUBE_NONE;
            }
        };
        
        if selection.is_selected {
            //let m = materials.get_mut(mat_handle).unwrap();
            //m.emissive = EXPLORE_CUBE_SELECTED;
            page.selections.insert(cube.entity_id);
        }
    }

    let mut show_fragment_ids = page.selections.iter().map(|x|x.clone()).collect::<Vec<_>>();

    if let Some(hover_id_unwrap) = page.hover_id {
        show_fragment_ids.push(hover_id_unwrap);
    }

    let mut show_fragment_contents = Vec::new();

    for id in show_fragment_ids {
        if let Ok(f) = q_fragment.get(id) {
            show_fragment_contents.push(f.contents.clone());
        }

        if let Ok(l) = q_entry.get(id) {
            for e in l.entities.clone() {
                show_fragment_contents.push(q_fragment.get(e).unwrap().contents.clone());
            }
        }
    }

    let hover_txt = show_fragment_contents.iter().map(|x| match x.clone() {
        FragmentContents::TextData { data } => {
            data
        },
        FragmentContents::Code { data, language } => todo!(),
        FragmentContents::URL { data } => todo!(),
        FragmentContents::Image { data } => todo!(),
    }).collect::<Vec<_>>().join("\n");

    let mut txt = q_text.single_mut();
    txt.sections[0].value = chunk_string(hover_txt, 25);
}

pub fn keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut q_cam: Query<&mut Transform, With<Camera3d>>
) {
    let mut tr = q_cam.single_mut();
    if keys.pressed(KeyCode::KeyQ) {
        tr.translation.z += 0.1;
    }
    if keys.pressed(KeyCode::KeyW) {
        tr.translation.z -= 0.1;
    }
}