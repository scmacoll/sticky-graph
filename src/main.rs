use eframe::{run_native, NativeOptions};
use egui::text::{CCursor, CCursorRange};
use egui::LayerId;
use egui::{
    vec2, Align, Button, CentralPanel, Color32, Context, CursorIcon, FontData, FontDefinitions,
    FontFamily, Frame as EguiFrame, Layout, Margin, Painter, Rect, ScrollArea, Sense, Stroke,
    StrokeKind, TextEdit, TopBottomPanel, Vec2, ViewportBuilder, ViewportCommand,
};
use std::fs;

const INCREMENT: f32 = 30.0;
const DEFAULT_SIZE: Vec2 = Vec2::new(200.0 + 2.0 * INCREMENT, 200.0 + 2.0 * INCREMENT);
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
        Box::new(|cc| {
            let data = fs::read("/Users/stu/Library/Fonts/Inter-Regular.ttf")
                .expect("Unable to load Inter font");
            let mut fonts = FontDefinitions::default();
            fonts
                .font_data
                .insert("Inter".into(), FontData::from_owned(data).into());
            fonts
                .families
                .get_mut(&FontFamily::Proportional)
                .unwrap()
                .insert(0, "Inter".into());
            fonts
                .families
                .get_mut(&FontFamily::Monospace)
                .unwrap()
                .insert(0, "Inter".into());
            cc.egui_ctx.set_fonts(fonts);
            cc.egui_ctx.set_pixels_per_point(DEFAULT_SCALE);
            Ok(Box::new(StickieApp::default()))
        }),
    )
}

struct StickieApp {
    text: String,
    window_size: Vec2,
    ui_scale: f32,
    should_focus: bool,
}

impl Default for StickieApp {
    fn default() -> Self {
        Self {
            text: String::new(),
            window_size: DEFAULT_SIZE,
            ui_scale: DEFAULT_SCALE,
            should_focus: true,
        }
    }
}

impl eframe::App for StickieApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        // One Dark background #282C34
        [40.0 / 255.0, 44.0 / 255.0, 52.0 / 255.0, 1.0]
    }

    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // Subtle white glow when focused
        if ctx.input(|i| i.raw.focused) {
            let painter: Painter = ctx.layer_painter(LayerId::background());
            let rect: Rect = ctx.screen_rect();
            painter.rect_stroke(
                rect,
                0.0,
                Stroke::new(1.0, Color32::from_white_alpha(50)),
                StrokeKind::Inside,
            );
        }

        let input = ctx.input(|i| i.clone());

        // Zoom controls
        if input.modifiers.command && input.key_pressed(egui::Key::Equals) {
            self.ui_scale += 0.1;
            ctx.set_pixels_per_point(self.ui_scale);
        }
        if input.modifiers.command && input.key_pressed(egui::Key::Minus) {
            self.ui_scale = (self.ui_scale - 0.1).max(0.5);
            ctx.set_pixels_per_point(self.ui_scale);
        }
        if input.modifiers.command && input.key_pressed(egui::Key::Num0) {
            self.ui_scale = DEFAULT_SCALE;
            ctx.set_pixels_per_point(self.ui_scale);
        }

        // Resize controls
        if input.modifiers.alt {
            if input.modifiers.shift && input.key_pressed(egui::Key::ArrowDown) {
                self.window_size.y += INCREMENT;
            } else if input.key_pressed(egui::Key::ArrowDown) {
                self.window_size.y += 10.0;
            }
            if input.modifiers.shift && input.key_pressed(egui::Key::ArrowUp) {
                self.window_size.y = (self.window_size.y - INCREMENT).max(150.0);
            } else if input.key_pressed(egui::Key::ArrowUp) {
                self.window_size.y = (self.window_size.y - 10.0).max(150.0);
            }
            if input.modifiers.shift && input.key_pressed(egui::Key::ArrowRight) {
                self.window_size.x += INCREMENT;
            } else if input.key_pressed(egui::Key::ArrowRight) {
                self.window_size.x += 10.0;
            }
            if input.modifiers.shift && input.key_pressed(egui::Key::ArrowLeft) {
                self.window_size.x = (self.window_size.x - INCREMENT).max(150.0);
            } else if input.key_pressed(egui::Key::ArrowLeft) {
                self.window_size.x = (self.window_size.x - 10.0).max(150.0);
            }
            if input.key_pressed(egui::Key::Equals) {
                self.window_size += vec2(INCREMENT, INCREMENT);
            }
            if input.key_pressed(egui::Key::Minus) {
                self.window_size.x = (self.window_size.x - INCREMENT).max(150.0);
                self.window_size.y = (self.window_size.y - INCREMENT).max(150.0);
            }
            ctx.send_viewport_cmd(ViewportCommand::InnerSize(self.window_size));
        }

        // Shortcuts: new, close, duplicate, reset size, drag
        if input.modifiers.command && input.key_pressed(egui::Key::N) {
            if let Ok(exe) = std::env::current_exe() {
                let _ = std::process::Command::new(exe).spawn();
            }
        }
        if input.modifiers.command && input.key_pressed(egui::Key::W) {
            ctx.send_viewport_cmd(ViewportCommand::Close);
        }
        if input.modifiers.command && input.key_pressed(egui::Key::D) {
            if let Ok(exe) = std::env::current_exe() {
                let _ = std::process::Command::new(exe).spawn();
            }
        }
        if input.modifiers.alt && input.key_pressed(egui::Key::Num0) {
            self.window_size = DEFAULT_SIZE;
            ctx.send_viewport_cmd(ViewportCommand::InnerSize(self.window_size));
        }
        if input.modifiers.command && input.pointer.primary_pressed() {
            ctx.send_viewport_cmd(ViewportCommand::StartDrag);
        }
        ctx.output_mut(|o| {
            o.cursor_icon = if input.modifiers.command && input.pointer.hover_pos().is_some() {
                CursorIcon::PointingHand
            } else {
                CursorIcon::Default
            };
        });

        // Title bar
        TopBottomPanel::top("title_bar")
            .exact_height(28.0)
            .frame(EguiFrame::NONE.inner_margin(Margin {
                left: 8,
                right: 8,
                top: 4,
                bottom: 4,
            }))
            .show(ctx, |ui| {
                let drag_rect = ui.max_rect();
                if ui
                    .interact(drag_rect, ui.id().with("drag_bar"), Sense::drag())
                    .dragged()
                {
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

        // Content with auto-focus & select-all on first display
        CentralPanel::default()
            .frame(EguiFrame::NONE.inner_margin(Margin {
                left: 8,
                right: 8,
                top: 8,
                bottom: 8,
            }))
            .show(ctx, |ui| {
                ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        let mut output = TextEdit::multiline(&mut self.text)
                            .frame(false)
                            .hint_text("Type your note here…")
                            .desired_rows(10)
                            .desired_width(ui.available_width())
                            .show(ui);
                        if self.should_focus {
                            // first focus...
                            output.response.request_focus();
                            // then select all
                            output.state.cursor.set_char_range(Some(CCursorRange::two(
                                CCursor::new(0),
                                CCursor::new(self.text.len()),
                            )));
                            output.state.store(ctx, output.response.id);
                            self.should_focus = false;
                        }
                    });
            });
    }
}
