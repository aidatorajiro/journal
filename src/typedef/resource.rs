//! Type definitions (Resource).
//! Resource is a bevy feature that act as a singleton global data storage.

use bevy::{prelude::*, utils::{HashMap, HashSet}};
use fdg_sim::Simulation;
use petgraph::{Graph, graph::NodeIndex};

use super::component::{Fragment};

/// Data for startup management
#[derive(Default, Debug, Resource)]
pub struct StartupManagement {
    pub state_file_checked: bool,
    pub load_graph_done: bool,
    pub state_file_nonexistent: bool
}

/// Graph data structures for the Journal.
#[derive(Default, Debug, Resource)]
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
    pub history_graph_ids: HashMap<Entity, NodeIndex>,
    /// fragment to entry map
    pub fragment_to_entry: HashMap<Entity, HashSet<Entity>>
}

/// State for newpage Page.
#[derive(Default, Debug, Resource)]
pub struct NewPageState {
    /// Entry id which the user is currently working on. If it is None, it means that the user is creating a new entry. Used in "new page".
    pub page_entry_ids: Vec<Entity>,
    /// Clone of the fragments within 
    pub entry_clone: Vec<FragmentClone>
}

#[derive(Default, Resource)]
/// State for Explore Page.
pub struct ExploreState {
    pub simulation: Option<Simulation<Entity, f32>>,
    pub selections: HashSet<Entity>,
    pub hover_id: Option<Entity>
}

/// A data structure that represents either (1) existing fragment or (2) modified fragment or (3) completely new fragment
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