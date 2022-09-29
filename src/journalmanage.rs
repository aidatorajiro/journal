pub mod systems {
    use bevy::prelude::*;

    use crate::typedef::event::AddJournal;

    pub fn handle_add_journal(mut events: EventReader<AddJournal>) {
        for ev in events.iter() {
            
        }
    }
}