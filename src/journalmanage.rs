pub mod systems {
    use bevy::{prelude::*};
    use crate::{typedef::{event::*, component::*, resource::GameState}, utils::utils::*};

    use super::inner::{add_entry, add_fragment};

    pub fn handle_add_fragments(
        mut events: EventReader<AddFragments>,
        mut commands: Commands,
        entitylist: Query<&EntityList>,
        mut global: ResMut<GameState>,
        mut fragment_query: Query<&mut Fragment>
    ) {
        for ev in events.iter() {
            let ts = create_timestamp();

            let ents: Vec<Entity> =
                ev.contents
                .iter()
                .map(|x| {
                    add_fragment(&mut commands, ts, x, &mut fragment_query, &mut global)
                })
                .collect();

            if let Some(x) = ev.entry {
                add_entry(&mut commands, &[entitylist.get(x).unwrap().entities.clone(), ents].concat(), ts, &mut global, &fragment_query.to_readonly());
            } else {
                add_entry(&mut commands, &ents, ts, &mut global, &fragment_query.to_readonly());
            };
        }
    }
}

mod inner {
    use bevy::prelude::*;
    use crate::typedef::{resource::*, component::*};

    pub fn add_fragment (
        commands: &mut Commands,
        ts: u64,
        fragment_contents: &FragmentContents,
        fragment_query: &mut Query<&mut Fragment>,
        global: &mut ResMut<GameState>
    ) -> Entity {
        let entid = commands.spawn().insert(Fragment {
            timestamp: ts,
            contents: fragment_contents.clone(),
            neighbor_graph_index: None,
            history_graph_index: None
        }).id();
        let mut qmut = fragment_query.get_mut(entid).unwrap();
        let neighbor_graph_index = global.neighbor_graph.add_node(entid);
        qmut.neighbor_graph_index = Some(neighbor_graph_index);
        let history_graph_index = global.history_graph.add_node(entid);
        qmut.history_graph_index = Some(history_graph_index);
        entid
    }

    pub fn add_entry(
        commands: &mut Commands,
        entities: &Vec<Entity>,
        ts: u64,
        global: &mut ResMut<GameState>,
        fragment_query: &Query<&Fragment>
    ) -> Entity {
        let id_entry = commands.spawn().insert(Entry {}).insert(EntityList { timestamp: ts, entities: entities.clone() }).id();

        if let Some(mut id_from) = entities.get(0) {
            for id_to in entities.iter().skip(1) {
                let a = fragment_query.get(*id_from).unwrap().neighbor_graph_index.unwrap();
                let b = fragment_query.get(*id_to).unwrap().neighbor_graph_index.unwrap();
                global.neighbor_graph.add_edge(a, b, id_entry);
                id_from = id_to;
            }
        }

        id_entry
    }
}