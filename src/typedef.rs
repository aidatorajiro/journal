pub mod component {
    //! Type definitions (Component).
    use bevy::{prelude::*, window::WindowId, utils::HashSet};
    use serde::{Serialize, Deserialize};

    /// Top Page button.
    #[derive(Component, PartialEq, Eq)]
    pub enum TopPageButton {
        NewPage,
        Explore,
        Linear,
        Migrate
    }

    /// parent of all top page contents
    #[derive(Component, PartialEq, Eq)]
    pub struct  TopPageContents;

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
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[serde(tag = "type")]
    pub enum FragmentContents {
        TextData { data: String },
        Code { data: String, language: String },
        URL { data: String },
        Image { data: String }
    }

    /// A component reperesenting a journal fragment, combining metadata and contents together
    #[derive(Component, Serialize, Deserialize, Debug)]
    pub struct Fragment {
        pub timestamp: u64,
        pub contents: FragmentContents
    }

    /// A list of entity with a timestamp when it is compiled.
    #[derive(Component, Serialize, Deserialize, Default, Debug)]
    pub struct EntityList {
        pub timestamp: u64,
        pub entities: Vec<Entity>
    }

    /// A component reperesenting a journal entry (A sequence of journal fragments). Use together with EntityList.
    #[derive(Component, Serialize, Deserialize, Default, Debug)]
    pub struct Entry;

    /// A component reperesenting a history about something (A sequence of something). Use together with EntityList.
    #[derive(Component, Serialize, Deserialize, Default, Debug)]
    pub struct History;

    /// A component reperesenting a tag.
    #[derive(Component, Serialize, Deserialize, Default, Debug)]
    pub struct Tag {
        pub name: String,
        pub entities: HashSet<Entity>,
        pub events: Vec<TagEvent>
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct TagEvent {
        pub timestamp: u64,
        pub entity: Entity,
        pub action: TagEventAction
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub enum TagEventAction { AddEntity, RemoveEntity }
}

pub mod resource {
    //! Type definitions (Resource).

    use bevy::{prelude::*, utils::HashMap};
    use petgraph::{Graph, graph::NodeIndex};
    use serde::*;
    
    /// Global Game State, aside from entity components.
    #[derive(Serialize, Deserialize, Default, Debug)]
    pub struct GameGraph {
        /// A graph with Fragment entity as a node, and Entry entity as a edge. Represents spacial continuation among fragments.
        pub neighbor_graph: Graph<Entity, Entity>,
        /// entity id to node id
        pub neighbor_graph_ids: HashMap<Entity, NodeIndex>,
        /// A graph with Fragment, Entry or History as a node.
        /// Must be a directed acyclic graph with no duplicate edge, as it represents chronological history how these entities are modified, splitted or merged to make a new entity.
        /// Must not have connection between Fragment and Entry, Entry and History, or Fragment and History, as this violates conceptual hierarchy.
        pub history_graph: Graph<Entity, ChangeType>,
        /// entity id to node id
        pub history_graph_ids: HashMap<Entity, NodeIndex>
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub enum ChangeType {
        Modify,
        Split,
        Merge
    }
}

pub mod state {
    #[derive(Debug, Clone, Eq, PartialEq, Hash)]
    pub enum AppState {
        Top,
        NewPage,
        Explore,
        Linear,
        Mitigate
    }
}

pub mod event {
    //! Type definitions (Events).

    use bevy::prelude::*;

    use super::component::FragmentContents;
    
    /// This event will yield a new entry, from scratch or based on an existing entry.
    #[derive(Debug)]
    pub struct AddFragments {
        /// List of texts to add.
        pub contents: Vec<FragmentContents>,
        /// None for creating a new entry from scratch. Some for append to an existing entry (please provide Entity ID for the entry).
        /// In both cases, a new entry will be created, because entries should be immutable.
        pub entry: Option<Entity>
    }
}