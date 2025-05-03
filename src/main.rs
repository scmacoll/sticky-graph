use eframe::{run_native, NativeOptions};
use egui::{
    self, vec2, Vec2, Layout, Align, Button, Sense, ViewportCommand,
    ScrollArea, Frame as EguiFrame, Margin, TextEdit, Key, CursorIcon,
    FontDefinitions, FontData, FontFamily, Color32, Stroke, CornerRadius,
};
use egui::viewport::ViewportBuilder;
use std::fs;

fn main() -> eframe::Result<()> {
    let min_size = vec2(150.0, 150.0);
    let initial_size = vec2(200.0, 200.0);

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
            // Load Inter font at runtime
            let data = fs::read("/Users/stu/Library/Fonts/Inter-Regular.ttf")
                .expect("Unable to load Inter font");
            let mut fonts = FontDefinitions::default();
            fonts.font_data.insert(
                "Inter".to_owned(),
                FontData::from_owned(data).into(),
            );
            fonts
                .families
                .get_mut(&FontFamily::Proportional)
                .unwrap()
                .insert(0, "Inter".to_owned());
            fonts
                .families
                .get_mut(&FontFamily::Monospace)
                .unwrap()
                .insert(0, "Inter".to_owned());
            cc.egui_ctx.set_fonts(fonts);

            Ok(Box::new(StickieApp::default()))
        }),
    )
}

struct StickieApp {
    text: String,
    window_size: Vec2,
}

impl Default for StickieApp {
    fn default() -> Self {
        Self {
            text: String::new(),
            window_size: vec2(200.0, 200.0),
        }
    }
}

impl eframe::App for StickieApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let input = ctx.input(|i| i.clone());

        // Alt/Alt+Shift + Arrows for resizing
        if input.modifiers.alt {
            // Vertical resize
            if input.modifiers.shift && input.key_pressed(Key::ArrowDown) {
                self.window_size.y += 30.0;
            } else if input.key_pressed(Key::ArrowDown) {
                self.window_size.y += 10.0;
            }
            if input.modifiers.shift && input.key_pressed(Key::ArrowUp) {
                self.window_size.y = (self.window_size.y - 30.0).max(150.0);
            } else if input.key_pressed(Key::ArrowUp) {
                self.window_size.y = (self.window_size.y - 10.0).max(150.0);
            }
            // Horizontal resize
            if input.modifiers.shift && input.key_pressed(Key::ArrowRight) {
                self.window_size.x += 30.0;
            } else if input.key_pressed(Key::ArrowRight) {
                self.window_size.x += 10.0;
            }
            if input.modifiers.shift && input.key_pressed(Key::ArrowLeft) {
                self.window_size.x = (self.window_size.x - 30.0).max(150.0);
            } else if input.key_pressed(Key::ArrowLeft) {
                self.window_size.x = (self.window_size.x - 10.0).max(150.0);
            }
            // Alt + = / Alt + -
            if input.key_pressed(Key::Equals) {
                self.window_size += vec2(30.0, 30.0);
            }
            if input.key_pressed(Key::Minus) {
                self.window_size.x = (self.window_size.x - 30.0).max(150.0);
                self.window_size.y = (self.window_size.y - 30.0).max(150.0);
            }
            ctx.send_viewport_cmd(ViewportCommand::InnerSize(self.window_size));
        }

        // Cmd+N to spawn new stickie
        if input.modifiers.command && input.key_pressed(Key::N) {
            if let Ok(exe) = std::env::current_exe() {
                let _ = std::process::Command::new(exe).spawn();
            }
        }

        // Cmd + 0: reset the sticky to its default 200×200 size
        if input.modifiers.command && input.key_pressed(Key::Num0) {
            self.window_size = vec2(200.0, 200.0);
            ctx.send_viewport_cmd(ViewportCommand::InnerSize(self.window_size));
        }

        // Cmd+click anywhere to drag note
        if input.modifiers.command && input.pointer.primary_pressed() {
            ctx.send_viewport_cmd(ViewportCommand::StartDrag);
        }

        // Change cursor to hand on Cmd+hover
        ctx.output_mut(|out| {
            out.cursor_icon = if input.modifiers.command && input.pointer.hover_pos().is_some() {
                CursorIcon::PointingHand
            } else {
                CursorIcon::Default
            };
        });

        // Draw yellow sticky background
        let painter = ctx.layer_painter(egui::LayerId::background());
        let rect = ctx.screen_rect();
        painter.rect_filled(
            rect,
            CornerRadius::same(0), // square corners
            Color32::from_rgb(242, 232, 130),
        );

        // Top bar with drag & close
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

        // Content area with padding & auto-scroll
        egui::CentralPanel::default()
            .show(ctx, |ui| {
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
