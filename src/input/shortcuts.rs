// src/input/shortcuts.rs
use egui::{Context, Key};

/// All user‐facing commands in the app.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    ZoomIn,
    ZoomOut,
    ZoomReset,

    Resize(ResizeDirection, bool /* shift? */),
    ResetSize,

    NewWindow,
    CloseWindow,
    Duplicate,

    StartDrag,

    CopyAll,
    HideOthers,
    FocusEditor,
}

/// Which way to resize when Alt+Arrow or ± is pressed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResizeDirection {
    Up,
    Down,
    Left,
    Right,
    Both,
}

/// Inspect the current input state and return *one* Command (if any).
pub fn detect(ctx: &Context) -> Option<Command> {
    let input = ctx.input(|i| i.clone());

    // ==== Command (⌘) shortcuts ====
    if input.modifiers.command {
        if input.key_pressed(Key::Equals) {
            return Some(Command::ZoomIn);
        }
        if input.key_pressed(Key::Minus) {
            return Some(Command::ZoomOut);
        }
        if input.key_pressed(Key::Num0) {
            return Some(Command::ZoomReset);
        }

        if input.key_pressed(Key::N) {
            return Some(Command::NewWindow);
        }
        if input.key_pressed(Key::W) {
            return Some(Command::CloseWindow);
        }
        if input.key_pressed(Key::D) {
            return Some(Command::Duplicate);
        }

        if input.modifiers.shift && input.key_pressed(Key::C) {
            return Some(Command::CopyAll);
        }
        if input.modifiers.shift && input.key_pressed(Key::H) {
            return Some(Command::HideOthers);
        }

        if input.pointer.primary_pressed() {
            return Some(Command::StartDrag);
        }
    }

    // ==== Alt (⌥) + arrows / ± / Num0 ====
    if input.modifiers.alt {
        if input.key_pressed(Key::Num0) {
            return Some(Command::ResetSize);
        }
        if input.key_pressed(Key::ArrowUp) {
            return Some(Command::Resize(ResizeDirection::Up, input.modifiers.shift));
        }
        if input.key_pressed(Key::ArrowDown) {
            return Some(Command::Resize(
                ResizeDirection::Down,
                input.modifiers.shift,
            ));
        }
        if input.key_pressed(Key::ArrowLeft) {
            return Some(Command::Resize(
                ResizeDirection::Left,
                input.modifiers.shift,
            ));
        }
        if input.key_pressed(Key::ArrowRight) {
            return Some(Command::Resize(
                ResizeDirection::Right,
                input.modifiers.shift,
            ));
        }
        if input.key_pressed(Key::Equals) {
            return Some(Command::Resize(ResizeDirection::Both, false));
        }
        if input.key_pressed(Key::Minus) {
            return Some(Command::Resize(ResizeDirection::Both, true));
        }
    }

    None
}
