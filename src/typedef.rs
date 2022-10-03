pub mod component {
    //! Type definitions (Component).
    use bevy::{prelude::*, window::WindowId, utils::HashSet};
    use petgraph::{graph::NodeIndex};
    use serde::{Serialize, Deserialize};

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
        pub contents: FragmentContents,
        pub neighbor_graph_index: Option<NodeIndex>,
        pub history_graph_index: Option<NodeIndex>
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

    use bevy::prelude::*;
    use petgraph::Graph;
    use serde::*;
    
    /// Global Game State, aside from entity components.
    #[derive(Serialize, Deserialize, Default, Debug)]
    pub struct GameState {
        /// A graph with Fragment entity as a node, and Entry entity as a edge. Represents continuation among fragments.
        pub neighbor_graph: Graph<Entity, Entity>,
        /// A graph with Fragment, Entry or History as a node.
        /// Must be a directed acyclic graph with no duplicate edge, as it represents chronological history how these entities are modified, splitted or merged to make a new entity.
        /// Must not have connection between Fragment and Entry, Entry and History, or Fragment and History, as this violates conceptual hierarchy.
        pub history_graph: Graph<Entity, ChangeType>
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub enum ChangeType {
        Modify,
        Split,
        Merge
    }
}

pub mod event {
    //! Type definitions (Events).

    use bevy::prelude::*;

    use super::component::FragmentContents;
    
    /// Add fragments to existing or new entry
    #[derive(Debug)]
    pub struct AddFragments {
        /// List of texts to add.
        pub contents: Vec<FragmentContents>,
        /// None for creating new entry. Some for append to existing entry.
        pub entry: Option<Entity>
    }
}