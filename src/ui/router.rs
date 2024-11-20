//! Router: handles page transition events.

use bevy::prelude::*;

use crate::typedef::{event::*, state::AppState};

pub fn event_listener (
    mut app_state: ResMut<NextState<AppState>>,
    mut ev_newpage: EventReader<JumpToNewPage>,
    mut ev_explore: EventReader<JumpToExplore>,
    mut ev_linear: EventReader<JumpToLinear>,
    mut ev_migrate: EventReader<JumpToMigrate>,
    mut ev_top: EventReader<JumpToTop>,
) {
    for _ in ev_newpage.read() {
        app_state.set(AppState::NewPage)
    }
    for _ in ev_explore.read() {
        app_state.set(AppState::Explore)
    }
    for _ in ev_linear.read() {
        app_state.set(AppState::Linear)
    }
    for _ in ev_migrate.read() {
        app_state.set(AppState::Migrate)
    }
    for _ in ev_top.read() {
        app_state.set(AppState::TopPage)
    }
}