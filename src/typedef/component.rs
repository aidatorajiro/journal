//! Type definitions (Component).
//! Components are main bevy feature that allows each [`Entity`] to have different kinds of data, and tagging/filtering for [`Entity`].

use bevy::utils::HashMap;
use bevy::{prelude::*, window::WindowId, utils::HashSet, reflect::FromReflect};
use petgraph::graph::NodeIndex;
use petgraph::graph::EdgeIndex;
use serde::{Serialize, Deserialize};
use bevy::ecs::reflect::ReflectMapEntities;
use bevy::ecs::entity::{MapEntities, EntityMap, MapEntitiesError};

#[derive(Component)]
pub struct MainCamera2D;

#[derive(Component)]
pub struct MainCamera3D;

/// Top Page button.
#[derive(Component, PartialEq, Eq)]
pub enum TopPageButton {
    NewPage,
    Explore,
    Linear,
    Migrate
}

/// New Page button.
#[derive(Component, PartialEq, Eq)]
pub enum NewPageButton {
    /// return to top page
    Return,
    /// Add texts to the entry
    AddTexts,
    /// Edit a fragment within the entry.
    Save
}

#[derive(Component, PartialEq, Eq)]
pub enum ExploreButton {
    Return,
    Merge
}

/// Contents on "new page" tab. Used to handle page switch.
#[derive(Component)]
pub struct NewPageContents {}

/// parent of all top page contents
#[derive(Component, PartialEq, Eq)]
pub struct TopPageContents;

#[derive(Component)]
pub struct ExploreContents;

#[derive(Component)]
pub struct ExploreCube {
    pub force_graph_index: NodeIndex,
    pub entity_id: Entity
}

#[derive(Component)]
pub struct ExploreEdge {
    pub force_edge_index: EdgeIndex
}

#[derive(Component)]
pub struct ExploreFragmentText;

// A subwindow.
#[derive(Component, Default)]
pub struct SubWindow {
    pub initialized: bool,
    pub window_id: Option<WindowId>
}

// A subwindow with memo field.
#[derive(Component, Default)]
pub struct MemoField {
    pub textarea: String
}

// A subwindow with blank page.
#[derive(Component, Default)]
pub struct BlankPage;








/// Content for journal fragment (e.g. a chunk of text, reference to local/remote images, URLs, programming codes, etc.)
#[derive(Reflect, Debug, Clone, Serialize, Deserialize)]
// #[serde(tag = "type")]
pub enum FragmentContents {
    TextData { data: String },
    Code { data: String, language: String },
    URL { data: String },
    Image { data: String }
}

impl Default for FragmentContents {
    fn default() -> Self {
        FragmentContents::TextData { data: "なんかおかしいみたいだね".into() }
    }
}

/// A component representing a journal fragment, combining metadata and contents together
#[derive(Component, Reflect, Default, Debug, Clone)]
#[reflect(Component)]
pub struct Fragment {
    pub timestamp: u64,
    pub contents: FragmentContents
}

/// A list of entity with a timestamp when it is compiled.
#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component, MapEntities)]
pub struct EntityList {
    pub timestamp: u64,
    pub entities: Vec<Entity>
}

impl MapEntities for EntityList {
    fn map_entities(&mut self, entity_map: &bevy::ecs::entity::EntityMap) -> Result<(), bevy::ecs::entity::MapEntitiesError> {
        for t in &mut self.entities {
            *t=entity_map.get(*t)?
        }
        Ok(())
    }
}

/// A component representing a journal entry (A sequence of journal fragments). Use together with EntityList.
#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct Entry;

/// A component representing a tag.
#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct Tag {
    pub name: String,
    pub entities: HashSet<Entity>,
    pub events: Vec<TagEvent>
}

#[derive(Reflect, Debug, FromReflect)]
pub struct TagEvent {
    pub timestamp: u64,
    pub entity: Entity,
    pub action: TagEventAction
}

#[derive(Reflect, Default, Debug, Clone, FromReflect)]
pub enum TagEventAction { #[default] AddEntity, RemoveEntity }

/// dummy hack for saving/loading GameGraph
#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component, MapEntities)]
pub struct GameGraphDummy {
    pub neighbor_graph: EncodedGraph<Entity, Entity>,
    pub history_graph: EncodedGraph<Entity, ()>,
    pub fragment_to_entry: HashMap<Entity, HashSet<Entity>>
}

pub type EncodedGraph<A, B> = (Vec<A>, Vec<(usize, usize, B)>);

impl MapEntities for GameGraphDummy {
    fn map_entities(&mut self, entity_map: &EntityMap) -> Result<(), MapEntitiesError> {
        for e in &mut self.neighbor_graph.0 {
            *e = entity_map.get(*e)?;
        }
        for (_, _, e) in &mut self.neighbor_graph.1 {
            *e = entity_map.get(*e)?;
        }

        for e in &mut self.history_graph.0 {
            *e = entity_map.get(*e)?;
        }

        let mut mapped_hm: HashMap<Entity, HashSet<Entity>> = HashMap::new();

        for (entry, hs) in &self.fragment_to_entry {
            let mapped_entry = entity_map.get(entry.clone())?;
            let mut mapped_hs: HashSet<Entity> = HashSet::new();
            for fragment in hs {
                let mapped_fragment = entity_map.get(*fragment)?;
                mapped_hs.insert(mapped_fragment);
            }
            mapped_hm.insert(mapped_entry, mapped_hs);
        }

        self.fragment_to_entry = mapped_hm;

        Ok(())
    }
}