
pub mod typedef {
    //! Type definitions.

    use std::collections::HashMap;

    use serde::{Serialize, Deserialize};

    /// Global Game State.
    #[derive(Default, Debug)]
    pub struct GameState {
        pub textarea: String,
        pub database: Database
    }

    /// Local Window State.
    #[derive(Default)]
    pub struct GameWindowState {
        pub initialized: bool
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