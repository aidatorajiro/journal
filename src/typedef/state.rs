//! App State definitions
//! App State is a bevy feature that helps the management of the conditions when systems are allowed to run.

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    LoadSaveData,
    TopPage,
    NewPage,
    Explore,
    Linear,
    Migrate
}