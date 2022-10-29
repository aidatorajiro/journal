pub mod systems {
    //! Event and Data management for Journal data structure - systems

    use bevy::{prelude::*};
    use crate::{typedef::{event::*, resource::GameGraph, resource::FragmentClone}, utils::utils::*};

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
        for ev in events.iter() {
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
}

mod inner {
    //! Event and Data management for Journal data structure - utility functions for fragment data/graph

    use bevy::prelude::*;
    use crate::typedef::{resource::*, component::*};

    pub fn add_fragment (
        commands: &mut Commands,
        ts: u64,
        fragment_contents: &FragmentContents,
        graph: &mut ResMut<GameGraph>,
        original_id: Option<Entity>
    ) -> Entity {
        let entid = commands.spawn().insert(Fragment {
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
        let id_entry = commands.spawn().insert(Entry {}).insert(EntityList { timestamp: ts, entities: entities.clone() }).id();

        // add the entry as a subject of history
        let id_node = graph.history_graph.add_node(id_entry);
        graph.history_graph_ids.insert(id_entry, id_node);

        if let Some(mut id_from) = entities.get(0) {
            for id_to in entities.iter().skip(1) {
                let a = graph.neighbor_graph_ids[id_from];
                let b = graph.neighbor_graph_ids[id_to];
                graph.neighbor_graph.add_edge(a, b, id_entry);
                id_from = id_to;
            }
        }

        id_entry
    }
}