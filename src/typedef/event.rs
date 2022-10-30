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