use egui::Color32;

/// One-Dark base background for note body
pub const BODY_BG: Color32 = Color32::from_rgb(40, 44, 52);
/// Top bar accent from One-Dark theme
pub const TOP_BAR_BG: Color32 = Color32::from_rgb(62, 68, 81);
/// Stroke color when window is focused (subtle glow)
pub const FOCUS_STROKE_COLOR: Color32 = Color32::from_white_alpha(5);
/// Width of focus glow
pub const GLOW_STROKE_WIDTH: f32 = 1.0;
/// Corner radius for rounded rectangles
pub const CORNER_RADIUS: f32 = 12.0;
/// Title and button text color
pub const TITLE_TEXT_COLOR: Color32 = Color32::from_rgb(171, 178, 191);
/// Button hover background (optional)
pub const BUTTON_HOVER_BG: Color32 = Color32::from_white_alpha(10);
