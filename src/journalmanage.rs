//! Journalmanage: Manages the app's main database (graph database) and save/load feature.

pub mod systems {
    //! Event and Data management for Journal data structure - systems

    use std::{path::Path, fs::{self, File}, io::Write};

    use bevy::{prelude::*, reflect::TypeRegistry, tasks::IoTaskPool, utils::HashSet};
    use crate::{typedef::{event::*, resource::GameGraph, resource::{FragmentClone, StartupManagement}, state::AppState, component::{GameGraphDummy, FragmentContents, Fragment, EntityList, Entry, Tag, TagEvent, TagEventAction}}, utils::basic::*, constants::save::STATE_FILE, journalmanage::inner::{convert_encoded_to_graph, convert_graph_to_encoded}};

    use super::inner::{add_entry, add_fragment};

    /// Push the list of FragmentClone (which is either id of existing fragment or data of new fragment) into the graph and entity database.
    /// This function will update `neighbor_graph` and `history_graph` so that its edge relationships matches the references of Entity IDs provided by the list of FragmentClone.
    /// Then, it will spawn an Entity with Entry component.
    pub fn handle_sync_fragments(
        mut events: EventReader<SyncFragments>,
        mut commands: Commands,
        mut graph: ResMut<GameGraph>,
        mut ev_done: EventWriter<SyncFragmentsDone>
    ) {
        for ev in events.read() {
            let ts = create_timestamp();

            let ents: Vec<Entity> =
                ev.entry_clone
                .iter()
                .map(|x| match x {
                    FragmentClone::NotModified { fragment_id } => {
                        fragment_id.clone()
                    },
                    FragmentClone::Modified { fragment, original_id } => {
                        add_fragment(&mut commands, fragment.timestamp, &fragment.contents, &mut graph, original_id.clone())
                    },
                })
                .collect();

            let e = add_entry(&mut commands, &ents, ts, &mut graph);

            for orig in ev.original_entries.clone() {
                let id_x = graph.history_graph_ids.get(&orig).unwrap().clone();
                let id_y = graph.history_graph_ids.get(&e).unwrap().clone();
                graph.history_graph.add_edge(id_x, id_y, ());
            }
            
            ev_done.send(SyncFragmentsDone { entry_id: e });
        }
    }

    /// Load scene
    pub fn load_scene_system(world: &mut World) {
        println!("Checking ron file...");
        if Path::new(STATE_FILE).exists() { // TODO: maybe use IoTaskPool?
            println!("Loading ron file...");
            fs::copy(Path::new(STATE_FILE), Path::new(STATE_FILE).with_extension("ron.".to_string() + &create_timestamp().to_string())).unwrap();
            let asset_server = world.resource_mut::<AssetServer>();
            let handle: Handle<DynamicScene> = asset_server.load(Path::new("..").join(STATE_FILE));
            //let mut spawner =  SceneSpawner::default();
            //let res = spawner.spawn_sync(world, handle);
            //println!("{:?}", res);
            let id = world.spawn(DynamicSceneBundle {
                scene: handle,
                ..default()
            }).id();
            println!("Root entity id: {:?}", id);
        } else {
            let id = world.spawn_empty().id();
            println!("Root entity id: {:?}", id);
            
            let mut startup = world.resource_mut::<StartupManagement>();
            startup.state_file_nonexistent = true;

            let mut stat = world.resource_mut::<NextState<AppState>>();
            stat.set(AppState::TopPage);
        }
        let mut startup = world.resource_mut::<StartupManagement>();
        startup.state_file_checked = true;
    }

    /// Load scene, additional steps for the graph
    pub fn load_graph_system(
        mut stat: ResMut<NextState<AppState>>,
        mut r: ResMut<GameGraph>,
        mut startup: ResMut<StartupManagement>,
        mut commands: Commands,
        q: Query<(Entity, &GameGraphDummy)>,
        q_test: Query<Entity, Without<GameGraphDummy>>,
    ) {
        let (e, d) = match q.get_single() {Ok(x) => x, _ => return};

        println!("Loading dummy state...");

        let (neighbor_graph, neighbor_graph_ids) = convert_encoded_to_graph(d.neighbor_graph.clone());
        let (history_graph, history_graph_ids) = convert_encoded_to_graph(d.history_graph.clone());
        
        *r = GameGraph {
            neighbor_graph,
            neighbor_graph_ids,
            history_graph,
            history_graph_ids,
            fragment_to_entry: d.fragment_to_entry.clone()
        };

        commands.entity(e).despawn();

        startup.load_graph_done = true;

        println!("{:?}", q_test.iter().collect::<Vec<_>>());
        
        stat.set(AppState::TopPage)
    }

    /// Save scene
    pub fn save_scene_system(world: &mut World) {
        if world.resource::<State<AppState>>().get().clone() == AppState::LoadSaveData { return; }
        if !world.is_resource_changed::<GameGraph>() { return; }

        println!("Saving state...");

        // Delete entities with no components

        let parent_id = world.component_id::<Parent>().unwrap();

        let mut q = world.query::<Entity>();
        let empty_entities = q.iter(world).filter(|x| {
            let cs = world.inspect_entity(x.clone());
            if cs.len() == 0 || (cs.len() == 1 && cs.get(0).unwrap().id() == parent_id) {
                return true
            } else {
                return false
            }
        }).collect::<Vec<_>>();

        println!("Found {} empty entity. Deleting...", empty_entities.len());
        
        for e in empty_entities {
            world.despawn(e);
        }
        
        /*

        for archetype in world.archetypes().iter()
        {
            if archetype.entities().contains(entity) { return Some(archetype.components()) }
        }*/

        // Spawn dummy graph

        let graph = world.get_resource::<GameGraph>().unwrap();

        let dummy = GameGraphDummy {
            neighbor_graph: convert_graph_to_encoded(graph.neighbor_graph.clone()),
            history_graph: convert_graph_to_encoded(graph.history_graph.clone()),
            fragment_to_entry: graph.fragment_to_entry.clone()
        };

        world.spawn(dummy);

        // Register types for saving

        let mut builder = DynamicSceneBuilder::from_world(&world);

        let scene =
            builder
            .allow::<Fragment>()
            .allow::<EntityList>()
            .allow::<Entry>()
            .allow::<Tag>()
            .allow::<GameGraphDummy>()
            .extract_entities(world.iter_entities().map(|x| x.id()))
            .build();

        let mut type_registry = TypeRegistry::default();
        type_registry.register::<HashSet<Entity>>();
        type_registry.register_type_data::<HashSet<Entity>, ReflectSerialize>();
        type_registry.register_type_data::<HashSet<Entity>, ReflectDeserialize>();
        type_registry.register::<Entity>();
        type_registry.register_type_data::<Entity, ReflectSerialize>();
        type_registry.register_type_data::<Entity, ReflectDeserialize>();
        type_registry.register::<FragmentContents>();
        type_registry.register_type_data::<FragmentContents, ReflectSerialize>();
        type_registry.register_type_data::<FragmentContents, ReflectDeserialize>();
        type_registry.register::<String>();
        type_registry.register_type_data::<String, ReflectSerialize>();
        type_registry.register_type_data::<String, ReflectDeserialize>();
        type_registry.register::<Fragment>();
        type_registry.register::<EntityList>();
        type_registry.register::<Entry>();
        type_registry.register::<Tag>();
        type_registry.register::<TagEvent>();
        type_registry.register::<TagEventAction>();
        type_registry.register::<GameGraphDummy>();

        // generate serialized data

        let serialized_scene = match scene.serialize(&type_registry) {Ok(x) => x, Err(x) => {println!("{:?}", x); return}};

        println!("Success! Cleaning...");

        let mut q = world.query::<(Entity, &GameGraphDummy)>();
        let result = world.despawn(q.single(world).0);
        
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
}

mod inner {
    //! Event and Data management for Journal data structure - utility functions for fragment data/graph

    use bevy::{prelude::*, utils::{HashMap, HashSet}};
    use petgraph::{Graph, graph::NodeIndex};
    use crate::typedef::{resource::*, component::*};
    use core::hash::Hash;

    pub fn add_fragment (
        commands: &mut Commands,
        ts: u64,
        fragment_contents: &FragmentContents,
        graph: &mut ResMut<GameGraph>,
        original_id: Option<Entity>
    ) -> Entity {
        let entid = commands.spawn(Fragment {
            timestamp: ts,
            contents: fragment_contents.clone()
        }).id();

        let a = graph.neighbor_graph.add_node(entid);
        graph.neighbor_graph_ids.insert(entid, a);

        //add the fragment as a subject of history
        let b = graph.history_graph.add_node(entid);
        graph.history_graph_ids.insert(entid, b);

        if let Some(oid) = original_id {
            let oid_nodeid = graph.history_graph_ids.get(&oid).unwrap().clone();
            graph.history_graph.add_edge(oid_nodeid, b, ());
        }

        entid
    }

    pub fn add_entry(
        commands: &mut Commands,
        entities: &Vec<Entity>,
        ts: u64,
        graph: &mut ResMut<GameGraph>
    ) -> Entity {
        let id_entry = commands.spawn(Entry {}).insert(EntityList { timestamp: ts, entities: entities.clone() }).id();

        // add the entry as a subject of history
        let id_node = graph.history_graph.add_node(id_entry);
        graph.history_graph_ids.insert(id_entry, id_node);

        if let Some(mut id_from) = entities.get(0) {
            if graph.fragment_to_entry.get(id_from).is_none() {
                graph.fragment_to_entry.insert(id_from.clone(), HashSet::new());
            }
            graph.fragment_to_entry.get_mut(id_from).unwrap().insert(id_entry);
            for id_to in entities.iter().skip(1) {
                if graph.fragment_to_entry.get(id_to).is_none() {
                    graph.fragment_to_entry.insert(id_to.clone(), HashSet::new());
                }
                graph.fragment_to_entry.get_mut(id_to).unwrap().insert(id_entry);
                let a = graph.neighbor_graph_ids[id_from];
                let b = graph.neighbor_graph_ids[id_to];
                graph.neighbor_graph.add_edge(a, b, id_entry);
                id_from = id_to;
            }
        }

        id_entry
    }

    /// Construct graph and "weight to index" map from EncodedGraph
    pub fn convert_encoded_to_graph<A: Hash + Eq + Clone, B> (eg: EncodedGraph<A, B>) -> (Graph<A, B>, HashMap<A, NodeIndex>) {

        let mut neighbor_graph: Graph<A, B> = Graph::new();
        let mut neighbor_graph_ids: HashMap<A, NodeIndex> = HashMap::new();
        let mut tmp: HashMap<usize, NodeIndex> = HashMap::new();

        let mut i = 0;
        for ent_neighbor in eg.0 {
            let node_id = neighbor_graph.add_node(ent_neighbor.clone());
            neighbor_graph_ids.insert(ent_neighbor, node_id);
            tmp.insert(i, node_id);
            i += 1;
        }

        for (id_1, id_2, edge) in eg.1 {
            neighbor_graph.add_edge(*tmp.get(&id_1).unwrap(), *tmp.get(&id_2).unwrap(), edge);
        }

        (neighbor_graph, neighbor_graph_ids)
    }

    /// Construct graph and "weight to index" map from EncodedGraph
    pub fn convert_graph_to_encoded<A: Hash + Eq + Clone, B: Clone> (g: Graph<A, B>) -> EncodedGraph<A, B> {
        let mut tmp: HashMap<NodeIndex, usize> = HashMap::new();
        let mut vec_a: Vec<A> = Vec::new();
        let mut counter = 0;

        for ni in g.node_indices() {
            let a = g.node_weight(ni).unwrap().clone();
            vec_a.push(a);
            tmp.insert(ni, counter);
            counter += 1;
        }

        let mut vec_b: Vec<(usize, usize, B)> = Vec::new();
        for ei in g.edge_indices() {
            let b = g.edge_weight(ei).unwrap().clone();
            let nodes = g.edge_endpoints(ei).unwrap();
            vec_b.push((*tmp.get(&nodes.0).unwrap(), *tmp.get(&nodes.1).unwrap(), b));
        }

        (vec_a, vec_b)
    }
}
