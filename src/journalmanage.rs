pub mod systems {
    //! Event and Data management for Journal data structure - systems

    use bevy::{prelude::*};
    use crate::{typedef::{event::*, component::*, resource::{GameGraph, FragmentClone}}, utils::utils::*};

    use super::inner::{add_entry, add_fragment};

    pub fn handle_add_fragments(
        mut events: EventReader<AddFragments>,
        mut commands: Commands,
        entitylist: Query<&EntityList>,
        mut graph: ResMut<GameGraph>
    ) {
        for ev in events.iter() {
            let ts = create_timestamp();

            let ents: Vec<Entity> =
                ev.contents
                .iter()
                .map(|x| {
                    add_fragment(&mut commands, ts, x,  &mut graph)
                })
                .collect();

            if let Some(x) = ev.entry {
                add_entry(&mut commands, &[entitylist.get(x).unwrap().entities.clone(), ents].concat(), ts, &mut graph);
            } else {
                add_entry(&mut commands, &ents, ts, &mut graph);
            };
        }
    }

    /// Push the list of FragmentClone (which is either id of existing fragment or data of new fragment) into the graph and entity database.
    pub fn handle_sync_fragments(
        mut events: EventReader<SyncFragments>,
        mut commands: Commands,
        mut graph: ResMut<GameGraph>
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
                    FragmentClone::Modified { fragment } => {
                        add_fragment(&mut commands, fragment.timestamp, &fragment.contents,  &mut graph)
                    },
                })
                .collect();

            add_entry(&mut commands, &ents, ts, &mut graph);
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
        graph: &mut ResMut<GameGraph>
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