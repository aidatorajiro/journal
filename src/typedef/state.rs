#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    LoadSaveData,
    TopPage,
    NewPage,
    Explore,
    Linear,
    Migrate
}