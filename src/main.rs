use eframe::{run_native, NativeOptions};
use egui::{
    self, vec2, Vec2, Layout, Align, Button, Sense, ViewportCommand,
    ScrollArea, Frame as EguiFrame, Margin, TextEdit, Key, CursorIcon,
    FontDefinitions, FontData, FontFamily, Color32,
};
use egui::viewport::ViewportBuilder;
use std::fs;

// How much we step for Alt+Shift+Arrows, etc.
const INCREMENT: f32 = 30.0;
// Default sticky note size: 200 + 2×INCREMENT = 260×260
const DEFAULT_SIZE: Vec2 = Vec2::new(200.0 + 2.0 * INCREMENT, 200.0 + 2.0 * INCREMENT);
// The default “zoom” (pixels per point) when you first open the app:
const DEFAULT_SCALE: f32 = 1.25;

fn main() -> eframe::Result<()> {
    let min_size = vec2(150.0, 150.0);
    let initial_size = DEFAULT_SIZE;

    let native_options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_decorations(false)
            .with_always_on_top()
            .with_transparent(false)
            .with_inner_size(initial_size)
            .with_min_inner_size(min_size),
        ..Default::default()
    };

    run_native(
        "Stickie Prototype",
        native_options,
        Box::new(|cc: &eframe::CreationContext| {
            // 1) Set up Inter
            let data = fs::read("/Users/stu/Library/Fonts/Inter-Regular.ttf")
                .expect("Unable to load Inter font");
            let mut fonts = FontDefinitions::default();
            fonts.font_data.insert("Inter".into(), FontData::from_owned(data).into());
            fonts.families.get_mut(&FontFamily::Proportional).unwrap()
                 .insert(0, "Inter".into());
            fonts.families.get_mut(&FontFamily::Monospace).unwrap()
                 .insert(0, "Inter".into());
            cc.egui_ctx.set_fonts(fonts);

            // 2) Force the default zoom on creation:
            cc.egui_ctx.set_pixels_per_point(DEFAULT_SCALE);

            Ok(Box::new(StickieApp::default()))
        }),
    )
}

struct StickieApp {
    text: String,
    window_size: Vec2,
    ui_scale: f32,
}

impl Default for StickieApp {
    fn default() -> Self {
        Self {
            text: String::new(),
            window_size: DEFAULT_SIZE,
            ui_scale: DEFAULT_SCALE,
        }
    }
}

impl eframe::App for StickieApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let input = ctx.input(|i| i.clone());

        // === ZOOM CONTROLS ===
        // Cmd + = : zoom in
        if input.modifiers.command && input.key_pressed(Key::Equals) {
            self.ui_scale += 0.1;
            ctx.set_pixels_per_point(self.ui_scale);
        }
        // Cmd + - : zoom out
        if input.modifiers.command && input.key_pressed(Key::Minus) {
            self.ui_scale = (self.ui_scale - 0.1).max(0.5);
            ctx.set_pixels_per_point(self.ui_scale);
        }
        // Cmd + 0 : reset zoom
        if input.modifiers.command && input.key_pressed(Key::Num0) {
            self.ui_scale = DEFAULT_SCALE;
            ctx.set_pixels_per_point(self.ui_scale);
        }

        // === WINDOW RESIZE CONTROLS (Alt + Arrows, etc.) ===
        if input.modifiers.alt {
            if input.modifiers.shift && input.key_pressed(Key::ArrowDown) {
                self.window_size.y += INCREMENT;
            } else if input.key_pressed(Key::ArrowDown) {
                self.window_size.y += 10.0;
            }
            if input.modifiers.shift && input.key_pressed(Key::ArrowUp) {
                self.window_size.y = (self.window_size.y - INCREMENT).max(150.0);
            } else if input.key_pressed(Key::ArrowUp) {
                self.window_size.y = (self.window_size.y - 10.0).max(150.0);
            }
            if input.modifiers.shift && input.key_pressed(Key::ArrowRight) {
                self.window_size.x += INCREMENT;
            } else if input.key_pressed(Key::ArrowRight) {
                self.window_size.x += 10.0;
            }
            if input.modifiers.shift && input.key_pressed(Key::ArrowLeft) {
                self.window_size.x = (self.window_size.x - INCREMENT).max(150.0);
            } else if input.key_pressed(Key::ArrowLeft) {
                self.window_size.x = (self.window_size.x - 10.0).max(150.0);
            }
            if input.key_pressed(Key::Equals) {
                self.window_size += vec2(INCREMENT, INCREMENT);
            }
            if input.key_pressed(Key::Minus) {
                self.window_size.x = (self.window_size.x - INCREMENT).max(150.0);
                self.window_size.y = (self.window_size.y - INCREMENT).max(150.0);
            }
            ctx.send_viewport_cmd(ViewportCommand::InnerSize(self.window_size));
        }

        // Cmd+N → new sticky
        if input.modifiers.command && input.key_pressed(Key::N) {
            if let Ok(exe) = std::env::current_exe() {
                let _ = std::process::Command::new(exe).spawn();
            }
        }

        // Alt + 0: reset to DEFAULT_SIZE
        if input.modifiers.alt && input.key_pressed(Key::Num0) {
            self.window_size = DEFAULT_SIZE;
            ctx.send_viewport_cmd(ViewportCommand::InnerSize(self.window_size));
        }

        // Cmd+click → drag
        if input.modifiers.command && input.pointer.primary_pressed() {
            ctx.send_viewport_cmd(ViewportCommand::StartDrag);
        }

        // Cmd+hover → hand cursor
        ctx.output_mut(|o| {
            o.cursor_icon = if input.modifiers.command && input.pointer.hover_pos().is_some() {
                CursorIcon::PointingHand
            } else {
                CursorIcon::Default
            };
        });

        // Draw background
        let painter = ctx.layer_painter(egui::LayerId::background());
        painter.rect_filled(
            ctx.screen_rect(),
            0.0,
            Color32::from_rgb(242, 232, 130),
        );

        // Top bar
        egui::TopBottomPanel::top("title_bar")
            .exact_height(24.0)
            .show(ctx, |ui| {
                let drag_rect = ui.max_rect();
                if ui.interact(drag_rect, ui.id().with("drag_bar"), Sense::drag()).dragged() {
                    ctx.send_viewport_cmd(ViewportCommand::StartDrag);
                }
                ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                    ui.label("Stickie");
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if ui.add(Button::new("x").frame(false)).clicked() {
                            ctx.send_viewport_cmd(ViewportCommand::Close);
                        }
                    });
                });
            });

        // Content
        egui::CentralPanel::default().show(ctx, |ui| {
            EguiFrame::NONE
                .inner_margin(Margin { left: 8, right: 8, top: 4, bottom: 8 })
                .show(ui, |ui| {
                    ScrollArea::vertical()
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            let avail = ui.available_width();
                            ui.add(
                                TextEdit::multiline(&mut self.text)
                                    .frame(false)
                                    .hint_text("Type your note here…")
                                    .desired_rows(10)
                                    .desired_width(avail),
                            );
                        });
                });
        });
    }
}
