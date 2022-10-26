pub mod window {
    //! Constants Definition: Window
    pub const SECONDARY_EGUI_PASS: &str = "secondary_egui_pass";
}

pub mod style {
    //! Constants Definition: Color (such as UI color tone)
    use bevy::prelude::*;

    pub const TOPBTN_NORMAL: Color = Color::rgb(11.0/256.0, 11.0/256.0, 48.0/256.0);
    pub const TOPBTN_HOVER: Color = Color::rgb(15.0/256.0, 74.0/256.0, 175.0/256.0);
    pub const TOPBTN_TEXT_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);
    pub const TOPBTN_IMG_OVERLAY: Color = Color::rgba(1.0, 1.0, 1.0, 0.5);

    pub const NEWPAGE_NORMAL: Color = Color::rgb(145.0/256.0, 242.0/256.0, 200.0/256.0);
    pub const NEWPAGE_HOVER: Color = Color::rgb(195.0/256.0, 232.0/256.0, 46.0/256.0);
    pub const NEWPAGE_CLICK: Color = Color::rgb(230.0/256.0, 152.0/256.0, 44.0/256.0);

    pub const STATE_FILE: &str = "state.scn.ron";
}