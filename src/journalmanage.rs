pub mod systems {
    use bevy::{prelude::*};
    use crate::{typedef::{event::*, component::*, resource::GameState}, utils::utils::*};

    pub fn handle_add_journal(
        mut events: EventReader<AddToFragments>,
        mut commands: Commands,
        entitylist: Query<&EntityList>,
        mut global: ResMut<GameState>,
        mut fragment_query: Query<&mut Fragment>,
        fragment_query_r: Query<&Fragment>
    ) {
        for ev in events.iter() {
            let ts = create_timestamp();

            let ents: Vec<Entity> =
                ev.contents
                .iter()
                .map(|x| {
                    let entid = commands.spawn().insert(Fragment { timestamp: ts, contents: x.clone(), neighbor_graph_index: None, history_graph_index: None }).id();
                    let mut qmut = fragment_query.get_mut(entid).unwrap();
                    let neighbor_graph_index = global.neighbor_graph.add_node(entid);
                    qmut.neighbor_graph_index = Some(neighbor_graph_index);
                    let history_graph_index = global.history_graph.add_node(entid);
                    qmut.history_graph_index = Some(history_graph_index);
                    entid
                })
                .collect();

            if let Some(x) = ev.entry {
                add_entry(&mut commands, [entitylist.get(x).unwrap().entities.clone(), ents].concat(), ts, &mut global, &fragment_query_r);
            } else {
                add_entry(&mut commands, ents, ts, &mut global, &fragment_query_r);
            };
        }
    }

    fn add_entry(
        commands: &mut Commands,
        entities: Vec<Entity>,
        ts: u64,
        global: &mut ResMut<GameState>,
        fragment_query: &Query<&Fragment>
    ) {
        let id_entry = commands.spawn().insert(Entry {}).insert(EntityList { timestamp: ts, entities: entities.clone() }).id();

        if let Some(mut id_from) = entities.get(0) {
            for id_to in entities.iter().skip(1) {
                let a = fragment_query.get(*id_from).unwrap().neighbor_graph_index.unwrap();
                let b = fragment_query.get(*id_to).unwrap().neighbor_graph_index.unwrap();
                global.neighbor_graph.add_edge(a, b, id_entry);
                id_from = id_to;
            }
        }
    }
}
