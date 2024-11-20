//! App State definitions
//! App State is a bevy feature that helps the management of the conditions when systems are allowed to run.

use bevy::prelude::States;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum AppState {
    #[default] LoadSaveData,
    TopPage,
    NewPage,
    Explore,
    Linear,
    Migrate
}