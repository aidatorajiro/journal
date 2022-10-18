
//! UI definitions for explore
use bevy::prelude::*;
use crate::typedef::{state::AppState};

use super::inner::use_2d_camera;

pub fn migrate_systems_enter () -> SystemSet {
    return SystemSet::on_enter(AppState::Migrate).with_system(use_2d_camera).with_system(migrate_enter);
}

pub fn migrate_systems_exit () -> SystemSet {
    return SystemSet::on_enter(AppState::Migrate).with_system(migrate_exit);
}

pub fn migrate_systems_update () -> SystemSet {
    return SystemSet::on_enter(AppState::Migrate).with_system(migrate_update);
}

fn migrate_enter () {
    
}

fn migrate_exit () {

}

fn migrate_update () {

}