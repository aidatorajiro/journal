
use bevy::prelude::*;

use crate::typedef::{event::*, state::AppState};

pub fn ui_manage_systems () -> bevy::prelude::SystemSet {
    return SystemSet::new().with_system(event_listener)
}

fn event_listener (
    mut app_state: ResMut<State<AppState>>,
    mut ev_newpage: EventReader<JumpToNewPage>,
    mut ev_explore: EventReader<JumpToExplore>,
    mut ev_linear: EventReader<JumpToLinear>,
    mut ev_migrate: EventReader<JumpToMigrate>,
    mut ev_top: EventReader<JumpToTop>,
) {
    for x in ev_newpage.iter() {
        app_state.set(AppState::NewPage).expect("couldn't change state");
    }
    for x in ev_explore.iter() {
        app_state.set(AppState::Explore).expect("couldn't change state");
    }
    for x in ev_linear.iter() {
        app_state.set(AppState::Linear).expect("couldn't change state");
    }
    for x in ev_migrate.iter() {
        app_state.set(AppState::Migrate).expect("couldn't change state");
    }
    for x in ev_top.iter() {
        app_state.set(AppState::Top).expect("couldn't change state");
    }
}
