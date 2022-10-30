//! Type definitions (Events).
//! Events are used when bevy systems communicate each other. For example, it is used for page transition.

use bevy::prelude::*;

use super::{component::FragmentContents, resource::FragmentClone};

/// Send a request to add a new entry.
/// ID of the original entries and a list of FragmentClone, which is used to identify which fragment is modified from which, must be provided.
#[derive(Debug)]
pub struct SyncFragments {
    /// id of the original entries
    pub original_entries: Vec<Entity>,
    /// a list of FragmentClone
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