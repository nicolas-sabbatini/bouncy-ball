use bevy::prelude::Color;
// Screen config
pub const WIN_WIDTH: f32 = 800.0;
pub const WIN_HEIGHT: f32 = 600.0;
pub const WIN_TITLE: &str = "Pongy";

pub const WINDOWS_CAMERA_NAME: &str = "windows camera";
pub const WINDOWS_CAMERA_CLEAR_COLOR: Color = Color::Rgba {
    red: 0.3,
    green: 0.3,
    blue: 0.3,
    alpha: 1.0,
};

pub const GAME_CAMERA_NAME: &str = "game camera";
pub const GAME_CAMERA_TARGET_NAME: &str = "game camera target";
pub const GAME_CAMERA_CLEAR_COLOR: Color = Color::Rgba {
    red: 0.0,
    green: 0.0,
    blue: 0.0,
    alpha: 1.0,
};
