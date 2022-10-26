pub mod component {
    //! Type definitions (Component).
    use bevy::{prelude::*, window::WindowId, utils::HashSet, reflect::FromReflect};
    use serde::{Serialize, Deserialize};
    use bevy::ecs::reflect::ReflectMapEntities;
    use bevy::ecs::entity::MapEntities;
    
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

    /// Contents on "new page" tab. Used to handle page switch.
    #[derive(Component)]
    pub struct NewPageContents {}

    /// parent of all top page contents
    #[derive(Component, PartialEq, Eq)]
    pub struct TopPageContents;

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

    /// A component reperesenting a journal fragment, combining metadata and contents together
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
            for mut t in &mut self.entities {
                *t=entity_map.get(*t)?
            }
            Ok(())
        }
    }

    /// A component reperesenting a journal entry (A sequence of journal fragments). Use together with EntityList.
    #[derive(Component, Reflect, Default, Debug)]
    #[reflect(Component)]
    pub struct Entry;

    /// A component reperesenting a tag.
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
    #[reflect(Component)]
    pub struct GameGraphDummy {
        pub neighbor_graph: String,
        pub neighbor_graph_ids: String,
        pub history_graph: String,
        pub history_graph_ids: String
    }
}

pub mod resource {
    //! Type definitions (Resource).

    use bevy::{prelude::*, utils::HashMap};
    use petgraph::{Graph, graph::NodeIndex};

    use super::component::{Fragment};

    /// Data for startup management
    #[derive(Default, Debug)]
    pub struct StartupManagement {
        pub state_file_checked: bool,
        pub load_graph_done: bool,
        pub state_file_nonexistent: bool
    }
    
    /// Graph data structures for the Journal.
    #[derive(Default, Debug)]
    pub struct GameGraph {
        /// A graph with Fragment entity as a node, and Entry entity as a edge. Represents spacial continuation among fragments.
        pub neighbor_graph: Graph<Entity, Entity>,
        /// entity id to node id
        pub neighbor_graph_ids: HashMap<Entity, NodeIndex>,
        /// A graph with Fragment, Entry or History as a node.
        /// Must be a directed acyclic graph with no duplicate edge, as it represents chronological history how these entities are modified, splitted or merged to make a new entity.
        /// Must not have connection between Fragment and Entry, Entry and History, or Fragment and History, as this violates conceptual hierarchy.
        pub history_graph: Graph<Entity, ()>,
        /// entity id to node id
        pub history_graph_ids: HashMap<Entity, NodeIndex>
    }

    /// State for newpage Page.
    #[derive(Default, Debug)]
    pub struct NewPageState {
        /// Entry id which the user is currently working on. If it is None, it means that the user is creating a new entry. Used in "new page".
        pub page_entry_ids: Vec<Entity>,
        /// Clone of the fragments within 
        pub entry_clone: Vec<FragmentClone>
    }

    #[derive(Debug, Clone)]
    pub enum FragmentClone {
        /// pointer to the global data structure. (it means that the data has not been modified)
        NotModified { fragment_id: Entity },
        /// modified or newly added data (ready to be pushed to the database when syncing)
        Modified { fragment: Fragment, original_id: Option<Entity> }
    }

    #[derive(Debug, Clone)]
    pub struct SaveLoadManagement {
        pub nextsave: u64
    }

    impl Default for SaveLoadManagement {
        fn default() -> Self {
            Self { nextsave: 10000 }
        }
    }
}

pub mod state {
    #[derive(Debug, Clone, Eq, PartialEq, Hash)]
    pub enum AppState {
        LoadSaveData,
        TopPage,
        NewPage,
        Explore,
        Linear,
        Migrate
    }
}

pub mod event {
    //! Type definitions (Events).
    use bevy::prelude::*;

    use super::{component::FragmentContents, resource::FragmentClone};
    
    /// This event will yield a new entry, from scratch or based on an existing entry.
    #[derive(Debug)]
    pub struct AddFragments {
        /// List of texts to add.
        pub contents: Vec<FragmentContents>,
        /// None for creating a new entry from scratch. Some for append to an existing entry (please provide Entity ID for the entry).
        /// In both cases, a new entry will be created, because entries should be immutable.
        pub entry: Option<Entity>
    }

    #[derive(Debug)]
    pub struct SyncFragments {
        /// id of the original entries
        pub original_entries: Vec<Entity>,
        pub entry_clone: Vec<FragmentClone>
    }

    #[derive(Debug)]
    pub struct SyncFragmentsDone {
        pub entry_id: Entity
    }

    #[derive(Debug, Default)]
    pub struct JumpToNewPage {
        pub entry_ids: Vec<Entity>
    }

    #[derive(Debug, Default)]
    pub struct JumpToExplore {
    }

    #[derive(Debug, Default)]
    pub struct JumpToLinear {
    }

    #[derive(Debug, Default)]
    pub struct JumpToMigrate {
    }

    #[derive(Debug, Default)]
    pub struct JumpToTop {
    }
}