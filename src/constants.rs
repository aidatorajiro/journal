pub mod window {
    //! Constants Definition
    pub const SECONDARY_EGUI_PASS: &str = "secondary_egui_pass";
}

pub mod style {
    //! Color Definitions
    use bevy::prelude::*;

    pub const TOPBTN_NORMAL: Color = Color::rgb(11.0/256.0, 11.0/256.0, 48.0/256.0);
    pub const TOPBTN_HOVER: Color = Color::rgb(15.0/256.0, 74.0/256.0, 175.0/256.0);
    pub const TOPBTN_PRESSED: Color = Color::rgb(13.0/256.0, 181.0/256.0, 91.0/256.0);
    pub const TOPBTN_TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
    pub const ICON_BACKGROUND_COLOR: Color = Color::rgb(145.0/256.0, 242.0/256.0, 200.0/256.0);
}