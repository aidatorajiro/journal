pub mod component {
    //! Type definitions (Component).
    use bevy::{prelude::Component, window::WindowId};

    #[derive(Component, Default)]
    pub struct SwashText {
    }

    #[derive(Component, Default)]
    pub struct SubWindow {
        pub initialized: bool,
        pub window_id: Option<WindowId>
    }

    #[derive(Component, Default)]
    pub struct MemoField {
        pub textarea: String,
        pub tags: Vec<String>
    }

    #[derive(Component, Default)]
    pub struct BlankPage;


}

pub mod state {
    //! Type definitions (State).
    use std::collections::HashMap;

    use serde::{Serialize, Deserialize};

    #[derive(Default, Debug)]
    pub struct SimpleState {
        pub switch: bool
    }

    /// Global Game State.
    #[derive(Default, Debug)]
    pub struct GameState {
        pub database: Database
    }
    
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


pub mod event {
    //! Type definitions (Events).
    #[derive(Default, Debug)]
    pub struct AddJournal {
        pub text: String
    }
}