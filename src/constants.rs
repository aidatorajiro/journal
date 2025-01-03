//! Constants: constant definitions, such as color definitions.

pub mod window {
    //! Constants Definition: Window
    pub const SECONDARY_EGUI_PASS: &str = "secondary_egui_pass";
}

pub mod style {
    //! Constants Definition: Color (such as UI color tone)

    use bevy::{color::Color, math::Vec3};

    pub const TOPBTN_NORMAL: Color = Color::srgb(11.0/256.0, 11.0/256.0, 48.0/256.0);
    pub const TOPBTN_HOVER: Color = Color::srgb(15.0/256.0, 74.0/256.0, 175.0/256.0);
    pub const TOPBTN_TEXT_COLOR: Color = Color::srgb(1.0, 1.0, 1.0);
    pub const TOPBTN_IMG_OVERLAY: Color = Color::srgba(1.0, 1.0, 1.0, 0.5);

    pub const NEWPAGE_NORMAL: Color = Color::srgb(145.0/256.0, 242.0/256.0, 200.0/256.0);
    pub const NEWPAGE_HOVER: Color = Color::srgb(195.0/256.0, 232.0/256.0, 46.0/256.0);
    pub const NEWPAGE_CLICK: Color = Color::srgb(230.0/256.0, 152.0/256.0, 44.0/256.0);

    pub const EXPLORE_NORMAL: Color = Color::srgb(145.0/256.0, 242.0/256.0, 200.0/256.0);
    pub const EXPLORE_HOVER: Color = Color::srgb(195.0/256.0, 232.0/256.0, 46.0/256.0);
    pub const EXPLORE_CLICK: Color = Color::srgb(230.0/256.0, 152.0/256.0, 44.0/256.0);

    pub const EXPLORE_CUBE_CLICKED: Color = Color::srgb(195.0/256.0, 232.0/256.0, 46.0/256.0);
    pub const EXPLORE_CUBE_HOVERED: Color = Color::srgb(216.0/256.0, 114.0/256.0, 194.0/256.0);
    pub const EXPLORE_CUBE_NONE: Color = Color::srgb(3.0/256.0, 30.0/256.0, 7.0/256.0);
    pub const EXPLORE_CUBE_SELECTED: Color = Color::srgb(138.0/256.0, 206.0/256.0, 153.0/256.0);
    pub const EXPLORE_TEXT_COLOR: Color = Color::srgba(1.0, 1.0, 1.0, 0.4);
    pub const EXPLORE_CUBE_SIZE: Vec3 = Vec3::new(0.1, 0.1, 0.1);
}

pub mod save {
    pub const STATE_FILE: &str = "state.scn.ron";
}