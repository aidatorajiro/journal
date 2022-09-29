pub mod component {
    //! Type definitions (Component).
    use std::collections::HashMap;

    use bevy::{prelude::Component, window::WindowId};
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

    /// A part of the structure for the journal database.
    #[derive(Serialize, Deserialize, Default, Debug)]
    pub struct JournalEntryMetadata {
        pub timestamp: i64,
        pub tags: Vec<String>
    }

    /// A part of the structure for the journal database.
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(tag = "type")]
    pub enum JournalFragment {
        TextData { data: String },
        Code {data: String, language: String },
        URL { data: String },
        Image { data: String }
    }

    /// A part of the structure for the journal database.
    #[derive(Serialize, Deserialize, Default, Debug)]
    pub struct JournalEntry {
        pub metadata: JournalEntryMetadata,
        pub contents: Vec<JournalFragment>,
    }

    /// The journal database type definition.
    #[derive(Serialize, Deserialize, Default, Debug)]
    pub struct Database {
        pub original: HashMap<String, JournalEntry>,
        pub decomposed: HashMap<String, JournalEntry>,
        pub reassembled: HashMap<String, JournalEntry>
    }
}

pub mod resource {
    //! Type definitions (Resource).
    
    /// Global Game State, aside from entity components.
    #[derive(Default, Debug)]
    pub struct GameState {}
}


pub mod event {
    //! Type definitions (Events).
    #[derive(Default, Debug)]
    pub struct AddJournal {
        pub text: String,
        pub timestamp: u64
    }
}