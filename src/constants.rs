pub mod constants {
    //! Constants Definition
    use bevy::{window::WindowId, render::once_cell::sync::Lazy};
    
    pub static SECOND_WINDOW_ID: Lazy<WindowId> = Lazy::new(WindowId::new);
    pub const SECONDARY_EGUI_PASS: &str = "secondary_egui_pass";
}