
//! UI definitions for explore page
use bevy::prelude::*;
use crate::typedef::{state::AppState};

use super::inner::use_3d_camera;

pub fn explore_systems_enter () -> SystemSet {
    return SystemSet::on_enter(AppState::Explore).with_system(use_3d_camera).with_system(explore_enter);
}

pub fn explore_systems_exit () -> SystemSet {
    return SystemSet::on_enter(AppState::Explore).with_system(explore_exit);
}

pub fn explore_systems_update () -> SystemSet {
    return SystemSet::on_enter(AppState::Explore).with_system(explore_update);
}

fn explore_enter () {
    
}

fn explore_exit () {

}

fn explore_update () {

}