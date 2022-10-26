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
    for _ in ev_newpage.iter() {
        app_state.set(AppState::NewPage).unwrap();
    }
    for _ in ev_explore.iter() {
        app_state.set(AppState::Explore).unwrap();
    }
    for _ in ev_linear.iter() {
        app_state.set(AppState::Linear).unwrap();
    }
    for _ in ev_migrate.iter() {
        app_state.set(AppState::Migrate).unwrap();
    }
    for _ in ev_top.iter() {
        app_state.set(AppState::TopPage).unwrap();
    }
}